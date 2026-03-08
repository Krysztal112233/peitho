#[derive(Debug, Clone)]
pub struct PeithoConfig {
    pub database_url: String,
    pub sandbox: SandboxConfig,
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub network_default: String,
    pub timeout_secs: u64,
}
