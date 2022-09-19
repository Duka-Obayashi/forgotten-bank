use diesel::prelude::*;
use crate::db::schema;

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub character_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = schema::accounts)]
pub struct NewAccount {
    pub character_id: i32
}