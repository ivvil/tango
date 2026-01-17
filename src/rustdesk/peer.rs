use hbb_common::bytes::Bytes;
use sqlx::database;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    db::{self, Database},
    error::{TangoError, TangoResult},
};

#[derive(Clone)]
pub struct Peer {
    pub socket_address: SocketAddr,
    pub rd_id: String,
    pub device_uuid: Bytes,
}

pub struct PeersCollection {
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    pub db: Database,
}

impl PeersCollection {
    pub async fn new(db: Database) -> TangoResult<Self> {
        Ok(Self {
            peers: Default::default(),
            db,
        })
    }

    pub async fn add(&mut self, peer: Peer) -> TangoResult<Peer> {
        let mut peer_map = self.peers.write().await;

        match peer_map.entry(peer.rd_id.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => Err(TangoError::PeerError(
                crate::error::PeerError::AlreadyExists,
            )),
            std::collections::hash_map::Entry::Vacant(e) => {
                self.db.create_peer(peer.clone()).await?;
                Ok(e.insert(peer).clone())
            }
        }
    }

    pub async fn delete_id(&mut self, id: String) -> TangoResult<()> {
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

	pub async fn get() {
		
	}
}
