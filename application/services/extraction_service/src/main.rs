use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};

use log::info;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::FutureProducer,
    ClientConfig,
};

use extraction_service::{
    config,
    messaging::{consumer_thread, KafkaMessageKind},
    rest_api,
};

fn kafka_message_callback(msg: KafkaMessageKind) {
    info!("{:?}", msg);
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
    consumer_thread::start(consumer, Box::new(kafka_message_callback));

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

fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(rest_api::index)
            .service(rest_api::healthcheck)
            .service(rest_api::collect),
    );
}
