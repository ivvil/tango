use std::{net::SocketAddr, pin::Pin};

use hbb_common::{ResultType, bytes::Bytes};
use sqlx::{PgPool, Pool, Postgres, Transaction, migrate::MigrateError};
use tracing_subscriber::registry::Data;

use crate::{
    error::{TangoError, TangoResult},
    rustdesk::peer::Peer,
};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> TangoResult<Self> {
        let pool = PgPool::connect_lazy(database_url)?;

        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> TangoResult<()> {
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .map_err(TangoError::Migration)
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    pub async fn create_peer(&self, peer: Peer) -> TangoResult<()> {
        sqlx::query!(
            "INSERT INTO peers (peer_id, address, uuid) VALUES ($1, $2, $3)",
            peer.socket_address.to_string(),
            peer.rd_id,
            peer.device_uuid.as_ref()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn select_peer_by_id(&self, id: String) -> TangoResult<Option<Peer>> {
        match sqlx::query!(
            "SELECT peer_id, address, uuid FROM peers WHERE peer_id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await? {
            Some(peer) => Ok(Some(Peer { socket_address: peer.address.parse()?, rd_id: peer.peer_id, device_uuid: peer.uuid.into() })),
            None => Ok(None),
        }

    }

    pub async fn remove_peer_by_uuid(&self, uuid: Bytes) -> TangoResult<()> {
        sqlx::query!("DELETE FROM peers WHERE uuid = $1", uuid.as_ref())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn remove_peer(&self, peer: Peer) -> TangoResult<()> {
        self.remove_peer_by_uuid(peer.device_uuid).await
    }
}
