use super::model::*;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Account>>")]
pub struct FetchAccounts;

#[derive(Message)]
#[rtype(result = "QueryResult<Account>")]
pub struct FetchAccount {
    pub account_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Account>")]
pub struct CreateAccount {
    pub email: String,
}