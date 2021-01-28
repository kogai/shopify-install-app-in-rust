-- Your SQL goes here
CREATE TABLE merchants (
  shop_domain varchar PRIMARY KEY,
  access_token varchar NOT NULL
);

ALTER TABLE merchants
  ADD CONSTRAINT shop_domain UNIQUE (shop_domain);

CREATE TABLE charges (
  charge_id varchar PRIMARY KEY,
  shop_domain varchar REFERENCES merchants (shop_domain)
);

