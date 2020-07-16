use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    Ok(())
}
