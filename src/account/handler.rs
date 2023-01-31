use super::model::*;
use crate::schema::accounts::dsl::*;
use crate::{account::message::*, db_utils::DbActor};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchAccounts> for DbActor {
    type Result = QueryResult<Vec<Account>>;

    fn handle(&mut self, _msg: FetchAccounts, _ctx: &mut Self::Context) -> Self::Result {
        let mut connection = self
            .0
            .get()
            .expect("Fetch Accounts: Unable to establish connection");

        // BUG: UUID data_type so swithing back to SERIAL
        accounts.get_results::<Account>(&mut connection)
    }
}

impl Handler<FetchAccount> for DbActor {
    type Result = QueryResult<Account>;

    fn handle(&mut self, msg: FetchAccount, _ctx: &mut Self::Context) -> Self::Result {
        let mut connection = self
            .0
            .get()
            .expect("Fetch Account: Unable to establish connection");

        accounts
            .find(msg.account_id)
            .get_result::<Account>(&mut connection)
    }
}

impl Handler<CreateAccount> for DbActor {
    type Result = QueryResult<Account>;

    fn handle(&mut self, msg: CreateAccount, _ctx: &mut Self::Context) -> Self::Result {
        let mut connection = self
            .0
            .get()
            .expect("Create Account: Unable to establish connection");

        let new_account = NewAccount { email: msg.email };

        diesel::insert_into(accounts)
            .values(new_account)
            .get_result::<Account>(&mut connection)
    }
}
