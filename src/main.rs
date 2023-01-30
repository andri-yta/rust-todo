use actix_web::{get, web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
mod todo;
use todo::service;

struct AppState {
    todo_entries: Mutex<Vec<TodoEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoEntry {
    id: Option<String>,
    date: DateTime<Utc>,
    title: String,
    status: TodoStatus,
}

#[derive(Serialize, Deserialize, Clone)]
enum TodoStatus {
    Pending,
    InProgress,
    Done
}

#[get("/")]
async fn index() -> String {
    "Good health!".to_string()
}

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todo_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(web::scope("/api").configure(service::config))
            .route("/health", web::get().to(get_health_status))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
