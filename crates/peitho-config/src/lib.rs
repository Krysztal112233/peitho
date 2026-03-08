use serde::Deserialize;
use std::path::Path;

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
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(Path::new("/etc/peitho/config.toml")).required(false))
            .add_source(config::File::from(Path::new("./config.toml")).required(false))
            .build()?;

        settings.try_deserialize()
    }
}
