use std::thread;

use crate::kafka_message::{KafkaByteMessage, KafkaBytes, KafkaMessage};
use futures::executor::block_on;
use log::{info, warn};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    Message,
};

pub fn start(consumer: StreamConsumer) {
    thread::spawn(move || loop {
        match block_on(consumer.recv()) {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<[u8]>() {
                    None => {}
                    Some(Ok(s)) => {
                        let message: KafkaMessage = s.into();

                        match message {
                            KafkaMessage::Extract(extract_message) => {
                                info!("{:?}", extract_message)
                            }
                            KafkaMessage::Other(other_message) => info!("{:?}", other_message),
                        }
                    }
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                    }
                };

                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    });
}
