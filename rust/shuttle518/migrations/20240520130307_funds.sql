-- Add migration script here
CREATE TABLE IF NOT EXISTS funds (
  id serial PRIMARY KEY,
  user_id INTEGER NOT NULL,
  code varchar(12) NOT NULL,
  buy_date date NOT NULL,
  price numeric NOT NULL,
  amount money NOT NULL
);