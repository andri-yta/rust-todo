use actix_web::{Responder, get, HttpResponse, web::{Data, self}};

use actix::Addr;

use crate::{db_utils::{AppState, DbActor}, account::message::FetchAccounts};

#[get("/accounts")]
async fn fetch_accounts(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchAccounts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No accounts found"),
        _ => HttpResponse::NotFound().json("Unable to retrieve accounts")
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(fetch_accounts);
}