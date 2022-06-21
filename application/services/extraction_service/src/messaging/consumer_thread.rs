use std::thread;

use futures::executor::block_on;
use log::{info, warn};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    Message,
};

use crate::messaging::{callback::Processor, KafkaMessage};

pub fn start(consumer: StreamConsumer, mut processor: Processor<KafkaMessage>) {
    thread::spawn(move || loop {
        match block_on(consumer.recv()) {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<[u8]>() {
                    None => {}
                    Some(Ok(s)) => {
                        let message: KafkaMessage = s.into();
                        processor.process_event(message);
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
