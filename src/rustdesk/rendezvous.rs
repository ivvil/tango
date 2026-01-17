use crate::{db::Database, error::TangoResult};

use super::peer::PeersCollection;

struct RendezvousServer {
    peers: PeersCollection,
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

impl RendezvousServer {
    pub async fn start(main_port: RendezvousServerPorts, db: Database) -> TangoResult<()> {
        let peers = PeersCollection::new(db);
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
