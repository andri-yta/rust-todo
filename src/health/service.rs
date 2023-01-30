use super::model::{HealthStatus};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;

#[get("/health")]
async fn get_health_status() -> impl Responder {
    let health = HealthStatus {
        status: "Healthy!".to_string(),
        date: Utc::now(),
    };
    HttpResponse::Ok().json(health)
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(get_health_status);
}
