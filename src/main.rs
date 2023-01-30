use actix_web::{get, web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod health;
mod todo;

struct AppState {
    todo_entries: Mutex<Vec<TodoEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoEntry {
    id: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    title: String,
    status: TodoStatus,
}

#[derive(Serialize, Deserialize, Clone)]
enum TodoStatus {
    Pending,
    InProgress,
    Done,
}

#[get("/")]
async fn index() -> String {
    "Welcome!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todo_entries: Mutex::new(vec![]),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(index)
            .service(
                web::scope("/api")
                    .configure(todo::service::config)
                    .configure(health::service::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
