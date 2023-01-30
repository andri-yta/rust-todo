use super::model::*;
use crate::{
    account::message::FetchAccounts,
    db_utils::{DbActor},
};
use actix::Handler;
use diesel::{self, prelude::*};
use crate::schema::accounts::{dsl::*};


impl Handler<FetchAccounts> for DbActor {
    type Result = QueryResult<Vec<Account>>;

    fn handle(&mut self, _msg: FetchAccounts, _ctx: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.get().expect("Fetch Accounts: Unable to establish connection");
    
        // BUG: UUID data_type so swithing back to SERIAL
        accounts.get_results::<Account>(&mut connection)
      }
}
