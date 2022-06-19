use std::thread;

use futures::executor::block_on;
use log::{warn, info};
use rdkafka::{consumer::{StreamConsumer, Consumer, CommitMode}, Message};

pub fn start(consumer: StreamConsumer) {
    thread::spawn(move || loop {
        match block_on(consumer.recv()) {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!(
                    "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    m.key(),
                    payload,
                    m.topic(),  
                    m.partition(),
                    m.offset(),
                    m.timestamp()
                );

                consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        });
}