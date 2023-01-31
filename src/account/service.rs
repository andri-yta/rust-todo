use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};

use actix::Addr;

use crate::{
    account::{message::*, model::CreateAccountParam},
    db_utils::{AppState, DbActor},
};

#[get("/accounts")]
async fn fetch_accounts(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchAccounts).await {
        Ok(Ok(result)) => HttpResponse::Ok().json(result),
        Ok(Err(_)) => HttpResponse::NotFound().json("No accounts found"),
        _ => HttpResponse::NotFound().json("Unable to retrieve accounts"),
    }
}

#[get("/accounts/{account_id}")]
async fn fetch_account(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let account_id = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchAccount { account_id }).await {
        Ok(Ok(result)) => HttpResponse::Ok().json(result),
        Ok(Err(_)) => {
            HttpResponse::NotFound().json(format!("No account with id {account_id} found"))
        }
        _ => HttpResponse::NotFound().json("Unable to retrieve accounts"),
    }
}

#[post("/accounts")]
async fn create_account(
    state: Data<AppState>,
    body: web::Json<CreateAccountParam>,
) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let email = body.email.to_owned();
    let email_ref = email.clone();

    match db.send(CreateAccount { email }).await {
        Ok(Ok(result)) => HttpResponse::Ok().json(result),
        Ok(Err(_)) => {
            HttpResponse::NotFound().json(format!("Account with {email_ref} already exists!"))
        }
        _ => HttpResponse::NotFound().json("Unable to retrieve accounts"),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(fetch_accounts)
        .service(fetch_account)
        .service(create_account);
}
