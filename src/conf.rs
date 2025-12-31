use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TangoConfig {
    version: u8,
    database_url: String,
    admin_default_username: String,
    admin_default_password: String,
}

impl ::std::default::Default for TangoConfig {
    fn default() -> Self {
        TangoConfig {
            version: 0,
            database_url: "".into(),
            admin_default_username: "admin".into(),
            admin_default_password: "tango".into(),
        }
    }
}

pub fn load_config() -> Result<TangoConfig, confy::ConfyError> {
    confy::load("tango", None)
}

pub fn save_config(config: &TangoConfig) -> Result<(), confy::ConfyError> {
    confy::store("tango", None, config)
}