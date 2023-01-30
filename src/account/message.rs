use super::model::*;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Account>>")]
pub struct FetchAccounts;