pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;

// use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

const DATABASE_URL: &str = env!("DATABASE_URL");

pub fn establish_connection() -> PgConnection {
  PgConnection::establish(&DATABASE_URL).unwrap()
}
