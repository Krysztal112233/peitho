use snafu::{Location, Snafu};

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ConfigError {
    #[snafu(display("failed to build config sources at {location}"))]
    BuildConfig {
        source: config::ConfigError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("failed to deserialize config values at {location}"))]
    DeserializeConfig {
        source: config::ConfigError,
        #[snafu(implicit)]
        location: Location,
    },
}
