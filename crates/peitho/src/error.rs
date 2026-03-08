use peitho_err::{ErrorCode, ErrorContext};
use snafu::prelude::*;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Snafu)]
pub enum AppError {
    #[snafu(display("{source}"))]
    Config {
        #[snafu(source(from(peitho_config::ConfigError, Box::new)))]
        source: Box<peitho_config::ConfigError>,
    },

    #[snafu(display("{code:?} in {component}.{operation}: {source}"))]
    Runtime {
        code: ErrorCode,
        component: &'static str,
        operation: &'static str,
        source: std::io::Error,
    },
}

impl From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        let context = ErrorContext::new("peitho", "shutdown_signal", false);
        Self::Runtime {
            code: ErrorCode::Internal,
            component: context.component,
            operation: context.operation,
            source,
        }
    }
}

impl From<peitho_config::ConfigError> for AppError {
    fn from(source: peitho_config::ConfigError) -> Self {
        Self::Config {
            source: Box::new(source),
        }
    }
}
