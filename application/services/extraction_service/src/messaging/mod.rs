pub mod callback;
pub mod consumer_thread;

mod message;

pub use message::{ExtractMessage, ExtractMessageBuilder, KafkaMessageKind};
