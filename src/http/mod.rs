use std::sync::Arc;

use axum::{Router, routing::{get, get_service}, serve};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use routes::webui::login::login;
use tracing::{info};

use crate::{conf::TangoConfig, db::Database, error::TangoError};

pub mod routes;
pub mod webui;

#[derive(Clone)]
pub struct HTTPState {
    pub db: Database,
    pub config: TangoConfig,
}

pub async fn start_http_server(addr: String, state: HTTPState) -> Result<(), TangoError> {
    let listener = TcpListener::bind(addr).await.map_err(TangoError::SockBind)?;

	if let Ok(addr) = listener.local_addr() {
		info!("Listening on http://{addr}/");
	};

	let routes = Router::new()
		.route("/login", get(login))
		.nest_service("/static", ServeDir::new("./static"))
		.with_state(Arc::new(state));

	axum::serve(listener, routes).await.map_err(TangoError::HttpServer)
}
