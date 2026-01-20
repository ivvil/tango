use std::net::SocketAddr;

use hbb_common::{
    protobuf::Message,
    rendezvous_proto::{RegisterPeerResponse, RendezvousMessage},
    udp::FramedSocket,
};
use tokio::net::{TcpListener, TcpStream};

use tracing::{info, trace};

use crate::{
    db::Database,
    error::{TangoError, TangoResult},
    rustdesk::peer::Peer,
};

use super::peer::PeersCollection;

struct RendezvousServer {
    peers: PeersCollection,
    ports: RendezvousServerPorts,
}

struct RendezvousServerPorts {
    pub main_port: i32,
    pub ws_port: i32,
    pub nat_port: i32,
}

impl RendezvousServerPorts {
    pub fn new(main_port: i32) -> Self {
        Self {
            main_port,
            ws_port: main_port + 2,
            nat_port: main_port - 1,
        }
    }
}

pub struct RendezvousServerListeners {
    pub main_listener: TcpListener,
    pub nat_listener: TcpListener,
    pub ws_listener: TcpListener,
}

impl RendezvousServer {
    pub async fn start(ports: RendezvousServerPorts, db: Database) -> TangoResult<()> {
        let peers = PeersCollection::new(db).await;

        let srv = Self { peers, ports };

        Ok(())
    }

    async fn main_io_loop(&mut self, listeners: RendezvousServerListeners) -> TangoResult<()> {
        // TODO Add relay checks
        loop {
            tokio::select!(
                res = listeners.main_listener.accept() => {
                match res {
                    Ok((stream, addr)) => {
                        stream.set_nodelay(true).ok();
                    },
                    Err(err) => {
                        tracing::error!("Main listener error: {}", err);
                        return Err(TangoError::IOError(crate::error::IOError::MainListener))
                    },
                }
            })
        }
    }

    async fn rendezvous_handler(
        &mut self,
        peer: Peer,
        msg: RendezvousMessage,
        addr: SocketAddr,
    ) -> TangoResult<Option<RendezvousMessage>> {
        if let Some(msg) = msg.union {
            match msg {
                hbb_common::rendezvous_proto::rendezvous_message::Union::RegisterPeer(
                    register_peer,
                ) => {
                    if !register_peer.id.is_empty() {
                        trace!("New peer: {} {}", &register_peer.id, &addr);
                        let ip_change = match self.peers.get(register_peer.id).await? {
                            Some(p) => {
                                let mut do_ip_change = false;

                                if p.socket_address.port() != 0 {
                                    do_ip_change = (addr.ip() != p.socket_address.ip())
                                        && !addr.ip().is_loopback();
                                };

                                if do_ip_change {
                                    Some(p.socket_address.to_string())
                                } else {
                                    None
                                }
                            }
                            None => None,
                        };

                        if let Some(p) = ip_change {
                            info!(
                                "IP Change for peer {}. Old: {} New: {}",
                                peer.peer_id, peer.socket_address, p
                            );
                        };

                        let mut msg = RendezvousMessage::new();
                        msg.set_register_peer_response(RegisterPeerResponse {
                            request_pk: true,
                            ..Default::default()
                        });

                        Ok(Some(msg))
                    } else {
                        Ok(None)
                    }
                }
                hbb_common::rendezvous_proto::rendezvous_message::Union::RegisterPeerResponse(
                    register_peer_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::PunchHoleRequest(
                    punch_hole_request,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::PunchHole(punch_hole) => {
                    todo!()
                }
                hbb_common::rendezvous_proto::rendezvous_message::Union::PunchHoleSent(
                    punch_hole_sent,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::PunchHoleResponse(
                    punch_hole_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::FetchLocalAddr(
                    fetch_local_addr,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::LocalAddr(local_addr) => {
                    todo!()
                }
                hbb_common::rendezvous_proto::rendezvous_message::Union::ConfigureUpdate(
                    config_update,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::RegisterPk(
                    register_pk,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::RegisterPkResponse(
                    register_pk_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::SoftwareUpdate(
                    software_update,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::RequestRelay(
                    request_relay,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::RelayResponse(
                    relay_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::TestNatRequest(
                    test_nat_request,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::TestNatResponse(
                    test_nat_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::PeerDiscovery(
                    peer_discovery,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::OnlineRequest(
                    online_request,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::OnlineResponse(
                    online_response,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::KeyExchange(
                    key_exchange,
                ) => todo!(),
                hbb_common::rendezvous_proto::rendezvous_message::Union::Hc(health_check) => {
                    todo!()
                }
                _ => return Err(TangoError::RendezvousError),
            }
        } else {
            return Err(TangoError::RendezvousError);
        }
    }

    async fn update_addr(
        &mut self,
        id: String,
        addr: SocketAddr,
        socket: &mut FramedSocket,
    ) -> TangoResult<()> {
        let mut ip_change = true;

        if let Some(old_peer) = self.peers.get(id).await? {
            ip_change = (addr.ip() != old_peer.socket_address.ip()) && !addr.ip().is_loopback()
        };

        Ok(())
    }
}

// let mut sock = FramedSocket::new("0.0.0.0:21116").await.inspect_err(|e| error!(net_error=%e, "Error binding UDP socket")).unwrap();
// let mut listener = new_listener("0.0.0.0:21116", false).await.inspect_err(|e| error!(net_error=%e, "Error binding to TCP socket")).unwrap();
// let mut rlistener = new_listener("0.0.0.0:21117", false).await.inspect_err(|e| error!(net_error=%e, "Error binding to relay TCP socket")).unwrap();

// let mut id_map = HashMap::new();
// let relay_server = std::env::var("IP").inspect_err(|e| error!(env_err=%e, "Error reading IP environment variable")).unwrap();
// let mut saved_stream: Option<FramedStream> = None;

// loop {
//     tokio::select! {
//         Some(Ok((bytes, addr))) = sock.next() => {
//             handle_udp(&mut sock, bytes, addr.into(), &mut id_map).await;
//         }
//         Ok((stream, addr)) = listener.accept() => {
//             let mut stream = FramedStream::from(stream, addr);
//             if let Some(Ok(bytes)) = stream.next_timeout(3000).await {
//                 if let Ok(msg_in) = RendezvousMessage::parse_from_bytes(&bytes) {
//                     match msg_in.union {
//                         Some(rendezvous_message::Union::PunchHoleRequest(ph)) => {
//                             info!{peer_id = %ph.id, "recived PunchHoleRequest"};
//                             if let Some(addr) = id_map.get(&ph.id) {
//                                 let mut msg_out = RendezvousMessage::new();
//                                 msg_out.set_request_relay(RequestRelay {
//                                     relay_server: relay_server.clone(),
//                                     ..Default::default()
//                                 });
//                                 sock.send(&msg_out, addr.clone()).await.ok();
//                                 saved_stream = Some(stream);
//                             }
//                         },
//                         Some(rendezvous_message::Union::RelayResponse(_)) => {
//                             info!{peer_addr = %addr, "recived RelayResponse"};
//                             let mut msg_out = RendezvousMessage::new();
//                             msg_out.set_relay_response(RelayResponse {
//                                 relay_server: relay_server.clone(),
//                                 ..Default::default()
//                             });

//                             if let Some(mut stream) = saved_stream.take() {
//                                 stream.send(&msg_out).await.ok();
//                                 if let Ok((stream_a, _)) = rlistener.accept().await {
//                                     let mut stream_a = FramedStream::from(stream_a, addr);
//                                     stream_a.next_timeout(3000).await;
//                                     if let Ok((stream_b, _)) = rlistener.accept().await {
//                                         let mut stream_b = FramedStream::from(stream_b, addr);
//                                         stream_b.next_timeout(3000).await;
//                                         relay(stream_a, stream_b, &mut sock, &mut id_map).await;
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }
// }
