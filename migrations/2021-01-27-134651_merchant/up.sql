-- Your SQL goes here
CREATE TABLE merchants (
  shop_domain VARCHAR PRIMARY KEY,
  access_token VARCHAR NOT NULL
);

CREATE TABLE charges (
  charge_id VARCHAR PRIMARY KEY,
  shop_domain VARCHAR REFERENCES merchants(shop_domain)
)
