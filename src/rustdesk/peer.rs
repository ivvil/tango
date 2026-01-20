use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use hbb_common::bytes::Bytes;
use nonzero::nonzero;
use sqlx::database;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::Instant};

use crate::{
    db::{self, Database},
    error::{TangoError, TangoResult},
};

use super::peer_id::PeerId;

#[derive(Clone)]
pub struct Peer {
    pub socket_address: SocketAddr,
    pub peer_id: PeerId,
    pub device_uuid: Bytes,
    pub reg_pk_rate_limiter: Arc<DefaultDirectRateLimiter>,
}

impl Default for Peer {
    fn default() -> Self {
        Self {
            socket_address: "0.0.0.0:0".parse().unwrap(),
            peer_id: PeerId::default(),
            device_uuid: Bytes::new(),
            reg_pk_rate_limiter: Arc::new(RateLimiter::direct(
                Quota::with_period(Duration::from_secs(6))
                    .unwrap()
                    .allow_burst(nonzero!(3u32)),
            )),
        }
    }
}

pub struct PeersCollection {
    peers: Arc<RwLock<HashMap<PeerId, Peer>>>, // TODO Implement proper caching
    pub db: Database,
}

impl PeersCollection {
    pub async fn new(db: Database) -> Self {
        Self {
            peers: Default::default(),
            db,
        }
    }

    pub async fn add(&mut self, peer: Peer) -> TangoResult<Peer> {
        let mut peer_map = self.peers.write().await;

        match peer_map.entry(peer.peer_id.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => Err(TangoError::PeerError(
                crate::error::PeerError::AlreadyExists,
            )),
            std::collections::hash_map::Entry::Vacant(e) => {
                self.db.create_peer(peer.clone()).await?;
                Ok(e.insert(peer).clone())
            }
        }
    }

    pub async fn delete_id(&mut self, id: PeerId) -> TangoResult<()> {
        let mut peer_map = self.peers.write().await;

        match peer_map.entry(id) {
            std::collections::hash_map::Entry::Occupied(e) => {
                self.db.remove_peer(e.remove()).await?;
                Ok(())
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                Err(TangoError::PeerError(crate::error::PeerError::DoesntExist))
            }
        }
    }

    pub async fn get(&mut self, id: PeerId) -> TangoResult<Option<Peer>> {
        let mut peer_map = self.peers.write().await;

        match peer_map.entry(id.clone()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(Some(occupied_entry.get().clone()))
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                match self.db.select_peer_by_id(id.clone().to_string()).await? {
                    Some(p) => {
                        peer_map.insert(id, p.clone());
                        Ok(Some(p))
                    }
                    None => Ok(None),
                }
            }
        }
    }
}
