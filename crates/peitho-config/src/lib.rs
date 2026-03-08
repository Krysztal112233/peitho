mod error;

pub use error::{ConfigError, Result};

use serde::Deserialize;
use snafu::prelude::*;
use std::path::Path;
use crate::error::{BuildConfigSnafu, DeserializeConfigSnafu};

#[derive(Debug, Clone, Deserialize)]
pub struct PeithoConfig {
    pub database: DatabaseConfig,
    pub sandbox: SandboxConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SandboxConfig {
    pub network_default: String,
    pub timeout_secs: u64,
}

impl PeithoConfig {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::from(Path::new("/etc/peitho/config.toml")).required(false))
            .add_source(config::File::from(Path::new("./config.toml")).required(false))
            .build()
            .context(BuildConfigSnafu)?;

        settings.try_deserialize().context(DeserializeConfigSnafu)
    }
}
