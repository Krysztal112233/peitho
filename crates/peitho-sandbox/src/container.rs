use crate::profile::SandboxProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    profile: SandboxProfile,
    healthy: bool,
}

impl Container {
    pub fn new(profile: SandboxProfile) -> Self {
        Self {
            profile,
            healthy: true,
        }
    }

    pub fn profile(&self) -> &SandboxProfile {
        &self.profile
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy
    }

    pub fn mark_unhealthy(&mut self) {
        self.healthy = false;
    }
}
