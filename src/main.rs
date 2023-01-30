#[macro_use] extern crate diesel;

use actix::{SyncArbiter};
use actix_web::{get, web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use dotenv::dotenv;
use std::env;

mod db_utils;
mod schema;

mod health;
mod account;


use db_utils::{get_pool, AppState, DbActor};

#[get("/")]
async fn index() -> String {
    "Welcome!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    let app_data = web::Data::new(AppState {
        db: db_addr.clone()
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(index)
            .service(
                web::scope("/api")
                    .configure(account::service::config)
                    .configure(health::service::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
