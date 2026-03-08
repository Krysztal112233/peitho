use snafu::Snafu;

#[derive(Debug, Snafu, Clone, PartialEq, Eq)]
pub enum SandboxProfileError {
    #[snafu(display("memory_bytes exceeds i64::MAX, got {memory_bytes}"))]
    MemoryTooLarge { memory_bytes: u64 },
    #[snafu(display("cpu_utilization_percent must be in 1..=100, got {cpu_utilization_percent}"))]
    InvalidCpuUtilization { cpu_utilization_percent: u8 },
}

#[derive(Debug, Snafu)]
pub enum SandboxPoolError {
    #[snafu(display("failed to build sandbox pool: {source}"))]
    BuildPool {
        source: deadpool::managed::BuildError,
    },
}
