-- Add migration script here
ALTER TABLE funds
ADD COLUMN IF NOT EXISTS tranche NUMERIC NOT NULL;