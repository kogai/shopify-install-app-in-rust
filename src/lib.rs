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
  PgConnection::establish(&DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL))
}

// pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
//   use schema::posts;

//   let new_post = NewPost { title, body };

//   diesel::insert_into(posts::table)
//     .values(&new_post)
//     .get_result(conn)
//     .expect("Error saving new post")
// }
