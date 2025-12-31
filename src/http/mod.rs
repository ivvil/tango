use axum::Router;

use crate::db::Database;

mod routes;

#[derive(Clone)]
pub struct HTTPState {
    pub db: Database,    
}

pub fn routes() -> Router<HTTPState> {
    Router::new()
}