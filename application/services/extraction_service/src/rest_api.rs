use std::{fmt::format, time::Duration};

use crate::kafka_message::{ExtractMessage, KafkaBytes, KafkaMessage};
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::info;
use rdkafka::{
    message::{OwnedHeaders, ToBytes},
    producer::{FutureProducer, FutureRecord},
};
use rkyv::AlignedVec;

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
    let value = KafkaMessage::Extract(ExtractMessage {
        name: format!("abc"),
        number: -1,
    });

    let message: Vec<u8> = value.into();

    let delivery_status = producer
        .send(
            FutureRecord::to("collect")
                .payload(&message)
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
