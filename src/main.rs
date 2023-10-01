use lm_sensors::prelude::*;
use lm_sensors::errors::Error;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

fn get_sensor_data() -> Result<String, Error> {
    let sensors = lm_sensors::Initializer::default().initialize()?;
    let mut body = String::new();

    for chip in sensors.chip_iter(None) {
        for feature in chip.feature_iter() {
            let name = feature.name().transpose()?.unwrap_or("N/A");
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.raw_value() {
                    body.push_str(
                        format!("#\n"
                        ).as_str()
                    );
                    body.push_str(
                        format!("{}_{}_{}_{} {}\n", 
                                chip.to_string().replace("-","_"), 
                                name.to_string().replace(" ","_"),
                                feature.to_string().replace(" ","_"), 
                                sub_feature.to_string().replace(" ","_"), 
                                value
                        ).as_str()
                    );
                }
            }
        }
    }
    Ok(body)
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    match get_sensor_data() {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    "ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // add logger
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(metrics)
    })
    .bind(("0.0.0.0", 8282))?
    .run()
    .await
}

