// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        character_id -> Int4,
    }
}

diesel::table! {
    asset_types (id) {
        id -> Int4,
        asset_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        value -> Nullable<Float8>,
        transaction_type -> Nullable<Varchar>,
        inserted_at -> Nullable<Timestamp>,
        asset_type_id -> Nullable<Int4>,
        account_id -> Nullable<Int4>,
    }
}

diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> asset_types (asset_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    asset_types,
    transactions,
);
