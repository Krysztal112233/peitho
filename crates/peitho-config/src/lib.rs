use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PeithoConfig {
    pub database: DatabaseConfig,
    pub sandbox: SandboxConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub network_default: String,
    pub timeout_secs: u64,
}

impl PeithoConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .add_source(config::File::from(Path::new("/etc/peitho/config.toml")).required(false));

        if let Ok(home_dir) = std::env::var("HOME") {
            builder = builder.add_source(
                config::File::from(PathBuf::from(home_dir).join(".config/peitho/config.toml"))
                    .required(false),
            );
        }

        let settings = builder
            .add_source(config::File::from(Path::new("./config.toml")).required(false))
            .build()?;

        let timeout_secs = settings
            .get_int("sandbox.timeout_secs")?
            .try_into()
            .map_err(|_| {
                config::ConfigError::Message(
                    "sandbox.timeout_secs must be a non-negative integer".to_owned(),
                )
            })?;

        Ok(Self {
            database: DatabaseConfig {
                url: settings.get_string("database.url")?,
            },
            sandbox: SandboxConfig {
                network_default: settings.get_string("sandbox.network_default")?,
                timeout_secs,
            },
        })
    }
}
