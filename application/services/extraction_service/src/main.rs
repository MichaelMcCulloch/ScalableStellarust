use std::{thread, time::Duration};

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use extraction_service::config;
use futures::{executor::block_on, Stream};
use log::{info, warn};
use rdkafka::{
    consumer::{self, CommitMode, Consumer, StreamConsumer},
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Extraction Service Prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/collect")]
async fn collect(producer: Data<FutureProducer>) -> impl Responder {
    let delivery_status = producer
        .send(
            FutureRecord::to("collect")
                .payload(&format!("Message from Extract"))
                .key(&format!("Key 0"))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            Duration::from_secs(10),
        )
        .await;
    match delivery_status {
        Ok(_) => HttpResponse::Ok().body("Sent!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_toml = std::fs::read_to_string("Config.toml")?;
    let config: config::Config = toml::from_str(&config_toml)?;
    let brokers_string = &format!("{}:{}", config.kafka.broker, config.kafka.port);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers_string)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("group.id", config.kafka.group_id)
        .create()
        .expect("Consumer creation failed");
    consumer.subscribe(&[&"collect"]).unwrap();

    let producer: Data<FutureProducer> = Data::new(
        ClientConfig::new()
            .set("bootstrap.servers", brokers_string)
            .set("message.timeout.ms", &format!("{}", config.kafka.timeout))
            .create()
            .expect("Producer Create Error"),
    );

    std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},extraction_service={},rdkafka={}",
            config.log.actix_web, config.log.service, config.log.rdkafka
        ),
    );
    env_logger::init();
    start_consumer_thread(consumer);

    let bind_address = format!("{}:{}", config.service.server, config.service.port);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(producer.clone())
            .configure(init)
    })
    .bind(bind_address)?
    .run()
    .await
}

fn start_consumer_thread(consumer: StreamConsumer) {
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

fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
            .service(collect),
    );
}
