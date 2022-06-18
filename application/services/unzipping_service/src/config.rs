use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub service: Service,
    pub kafka: Kafka,
    pub log: Log,
}

#[derive(Deserialize)]
pub struct Kafka {
    pub broker: String,
    pub port: u16,
    pub timeout: u16,
}

#[derive(Deserialize)]
pub struct Service {
    pub server: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Log {
    pub actix_web: String,
    pub service: String,
    pub rdkafka: String,
}
