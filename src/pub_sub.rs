// pubsub.rs
use tokio::sync::broadcast;

pub fn create_pubsub_channel() -> broadcast::Sender<String> {
    let (sender, _receiver) = broadcast::channel(100); // Buffer size 100 (adjustable)
    sender
}
