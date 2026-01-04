use std::error::Error;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::migrate::MigrateError;
use thiserror::Error;
use tokio::task::JoinError;

use crate::http::webui::templates::error::ErrorTemplate;

#[derive(Debug, Error)]
pub enum TangoError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),

	#[error("Database migration error")]
	Migration(#[from] MigrateError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Template rendering error")]
    Render(#[from] askama::Error),

	#[error("Error binding socket")]
	SockBind(#[from] std::io::Error),

	#[error("Error starting HTTP server")]
	HttpServer(#[source] std::io::Error),

	#[error("Error loading config")]
	Config(#[from] confy::ConfyError),

	#[error("Error running async task")]
	Join(#[from] JoinError)
}

impl IntoResponse for TangoError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            TangoError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
			TangoError::Migration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TangoError::Unauthorized => StatusCode::UNAUTHORIZED,
            TangoError::Forbidden => StatusCode::FORBIDDEN,
            TangoError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
			TangoError::SockBind(_) => StatusCode::INTERNAL_SERVER_ERROR,
			TangoError::HttpServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
			TangoError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
			TangoError::Join(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let tmplt = ErrorTemplate {
            shorterror: status.as_str(),
            error: status.canonical_reason().unwrap_or(""),
        };

        if let Ok(body) = tmplt.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
