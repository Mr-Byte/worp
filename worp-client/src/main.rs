use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();
}
