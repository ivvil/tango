use hbb_common::config::RENDEZVOUS_PORT;
use serde::{Deserialize, Serialize};

use crate::error::TangoError;

#[derive(Serialize, Deserialize, Clone)]
pub struct TangoConfig {
    pub version: u8,
    pub database_url: String,
    pub webui: WebUIConfig,
    pub rustdesksrv: RustdeskSrvConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebUIConfig {
    pub admin_default_username: String,
    pub admin_default_password: String,
    pub http_addr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RustdeskSrvConfig {
    pub port: i32,
}

impl ::std::default::Default for TangoConfig {
    fn default() -> Self {
        TangoConfig {
            version: 0,
            database_url: "".into(),
            webui: WebUIConfig {
                admin_default_username: "admin".into(),
                admin_default_password: "tango".into(),
                http_addr: "127.0.0.1:80120".into(),
            },
            rustdesksrv: RustdeskSrvConfig {
                port: RENDEZVOUS_PORT,
            },
        }
    }
}

pub fn load_config() -> Result<TangoConfig, TangoError> {
    confy::load("tango", None).map_err(TangoError::Config)
}

pub fn save_config(config: &TangoConfig) -> Result<(), TangoError> {
    confy::store("tango", None, config).map_err(TangoError::Config)
}
