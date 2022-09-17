CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  value FLOAT,
  transaction_type VARCHAR,
  inserted_at TIMESTAMP,
  asset_type_id INTEGER,
  account_id INTEGER,
  CONSTRAINT fk_transactions_accounts
    FOREIGN KEY (account_id)
    REFERENCES accounts (id),
  CONSTRAINT fk_transactions_asset_types
    FOREIGN KEY (asset_type_id)
    REFERENCES asset_types (id)
)