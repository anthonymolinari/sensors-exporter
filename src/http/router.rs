use actix_web::{HttpResponse, Responder, get};
use crate::data::sensors::get_sensor_data;

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    match get_sensor_data() {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    "ok"
}