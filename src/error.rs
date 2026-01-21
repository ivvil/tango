use std::{error::Error, net::AddrParseError};

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::migrate::MigrateError;
use thiserror::Error;
use tokio::task::JoinError;

use crate::{http::webui::templates::error::ErrorTemplate, rustdesk::peer_id::PeerIdError};

pub type TangoResult<T> = Result<T, TangoError>;

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
    Join(#[from] JoinError),

    #[error("Error parsing an IP Address")]
    IPParse(#[from] AddrParseError),

    #[error("Peer Error")]
    PeerError(#[from] PeerError),

	#[error("I/O Error")]
	IOError(IOError),

	#[error("Rendezvous protocol error")]
	RendezvousError,

	#[error("Doesn't exist")]
	DoesntExist
}

#[derive(Debug, Error)]
pub enum PeerError {
    #[error("Error peer already exists")]
    AlreadyExists,

	#[error("Error peer doesn't exist")]
	DoesntExist,

	#[error("ID Error")]
	IDError(#[from] PeerIdError),

    #[error("Register PK error")]
    RegisterPk,
}

#[derive(Debug, Error)]
pub enum IOError {
	#[error("I/O Error in the main listener")]
    MainListener
}


impl IntoResponse for TangoError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            TangoError::Unauthorized => StatusCode::UNAUTHORIZED,
            TangoError::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
