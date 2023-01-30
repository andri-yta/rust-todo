// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;
use diesel::{Queryable};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
#[diesel(primary_key(account_id))]
pub struct Account {
    pub account_id: i32,
    pub email: String,
}