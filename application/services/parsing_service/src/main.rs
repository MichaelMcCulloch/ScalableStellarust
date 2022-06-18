use std::time::Duration;

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use parsing_service::config;
use rdkafka::{
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Parsing Service Prototype")
}
pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
            .service(collect),
    );
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
                .payload(&format!("Message from Parsing"))
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

    // thread::sleep(Duration::from_secs(7));
    let producer: Data<FutureProducer> = Data::new(
        ClientConfig::new()
            .set(
                "bootstrap.servers",
                &format!("{}:{}", config.kafka.broker, config.kafka.port),
            )
            .set("message.timeout.ms", &format!("{}", config.kafka.timeout))
            .create()
            .expect("Producer Create Error"),
    );
    std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},parsing_service={},rdkafka={}",
            config.log.actix_web, config.log.service, config.log.rdkafka
        ),
    );
    env_logger::init();

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
