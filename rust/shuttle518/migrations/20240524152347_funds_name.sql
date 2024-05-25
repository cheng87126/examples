-- Add migration script here
ALTER TABLE funds
ADD COLUMN IF NOT EXISTS fnud_name varchar(100);