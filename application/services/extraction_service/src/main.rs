use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use extraction_service::config;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Extraction Service Prototype")
}
pub fn init(config: &mut web::ServiceConfig) {
    config.service(web::scope("").service(index).service(healthcheck));
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_toml = std::fs::read_to_string("Config.toml")?;
    let config: config::Config = toml::from_str(&config_toml)?;
    std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},extraction_service={}",
            config.log.actix_web, config.log.service
        ),
    );
    env_logger::init();

    let bind_address = format!("{}:{}", config.service.server, config.service.port);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(init)
    })
    .bind(bind_address)?
    .run()
    .await
}
