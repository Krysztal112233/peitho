#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    ConfigInvalid,
    DependencyUnavailable,
    BadRequest,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorContext {
    pub component: &'static str,
    pub operation: &'static str,
    pub retryable: bool,
}

impl ErrorContext {
    pub const fn new(component: &'static str, operation: &'static str, retryable: bool) -> Self {
        Self {
            component,
            operation,
            retryable,
        }
    }
}
