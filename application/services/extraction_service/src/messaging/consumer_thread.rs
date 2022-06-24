use std::{borrow::Borrow, thread};

use crate::messaging::{callback::Callback, KafkaMessageKind};
use futures::executor::block_on;
use log::{info, warn};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    Message,
};

pub fn start(consumer: StreamConsumer, mut processor: Callback<KafkaMessageKind>) {
    thread::spawn(move || loop {
        thread_action(&consumer, &mut processor);
    });
}

fn thread_action(consumer: &StreamConsumer, processor: &mut Callback<KafkaMessageKind>) {
    match block_on(consumer.recv()) {
        Err(e) => warn!("Kafka error: {}", e),
        Ok(m) => {
            match m.payload_view::<[u8]>() {
                None => {}
                Some(Ok(s)) => {
                    let message: KafkaMessageKind = s.into();
                    processor(message);
                }
                Some(Err(e)) => {
                    warn!("Error while deserializing message payload: {:?}", e);
                }
            };

            consumer.commit_message(&m, CommitMode::Async).unwrap();
        }
    };
}
