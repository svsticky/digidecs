use serde::{Deserialize, Serialize};
use crate::file::DataFile;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub smtp: SmtpConfig,
    pub treasurer_email: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    pub domain: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SmtpConfig {
    pub from_email: String,
    pub from_name: String,
    pub smtp_relay: String,
}

fn default_port() -> u16 {
    8080
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            domain: String::new(),
        }
    }
}

impl DataFile for AppConfig {}