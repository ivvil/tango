use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TangoConfig {
    version: u8,
    database_url: String,
}

impl ::std::default::Default for TangoConfig {
    fn default() -> Self {
        TangoConfig {
            version: 0,
            database_url: "".into(),
        }
    }
}

pub fn load_config() -> Result<TangoConfig, confy::ConfyError> {
    confy::load("tango", None)
}

pub fn save_config(config: &TangoConfig) -> Result<(), confy::ConfyError> {
    confy::store("tango", None, config)
}