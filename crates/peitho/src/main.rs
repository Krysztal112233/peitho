use std::io;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> io::Result<()> {
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    // wowowo, UNIX only thanks.
    loop {
        tokio::select! {
            _ = &mut shutdown => {
                println!("shutdown signal received, exiting now");
                break;
            }
            _ = sleep(Duration::from_secs(60)) => {
                // Keep process alive until a shutdown signal is received.
            }
        }
    }

    Ok(())
}

async fn shutdown_signal() -> io::Result<()> {
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(()),
        _ = sigterm.recv() => Ok(()),
    }
}
