pub mod models;

use super::*;
use self::models::{Account, NewAccount};
use diesel::prelude::*;
use self::schema::accounts::dsl::*;

pub fn list_accounts() -> Vec<Account> {

    let connection = &mut establish_connection();
    let results = accounts
        .load::<Account>(connection)
        .expect("Error loading posts");

    results

}

pub fn get_account_by_id(selected_id: i32) -> Vec<Account> {

    let connection = &mut establish_connection();
    let results = accounts
        .filter(id.eq(selected_id))
        .load::<Account>(connection)
        .expect("Error loading posts");

    results

}

pub fn get_account_by_character_id(selected_id: i32) -> Vec<Account> {

    let connection = &mut establish_connection();
    let results = accounts
        .filter(character_id.eq(selected_id))
        .load::<Account>(connection)
        .expect("Error loading posts");

    results

}

pub fn create_account(new_id: i32) -> Account {

    let connection = &mut establish_connection();
    let new_account = NewAccount { character_id: new_id };

    diesel::insert_into( self::schema::accounts::table)
        .values(&new_account)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn update_character(selected_id: i32, new_id: i32) -> Account {

    let connection = &mut establish_connection();

    let account = diesel::update(accounts.find(selected_id))
        .set(character_id.eq(new_id))
        .get_result::<Account>(connection)
        .unwrap();
    
    account

}

pub fn delete_account(account_id: i32) {

    let connection = &mut establish_connection();

    diesel::delete(accounts.filter(id.eq(account_id)))
        .execute(connection)
        .expect("Error deleting posts");

}


