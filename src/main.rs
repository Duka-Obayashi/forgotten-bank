use forgotten_bank::db::*;

fn main() {
    println!("Hello, world!");

    accounts::update_character(3, 555);

    let accounts = accounts::list_accounts();

    println!("Displaying {} posts", accounts.len());
    for account in accounts {
        println!("{} --> {}", account.id, account.character_id);
        println!("-----------\n");
    }

}
