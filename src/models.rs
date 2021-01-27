// use super::schema::posts;
use super::schema::merchants;

// #[derive(Insertable)]
// #[table_name = "posts"]
// pub struct NewPost<'a> {
//   pub title: &'a str,
//   pub body: &'a str,
// }

// #[derive(Queryable)]
// pub struct Post {
//   pub id: i32,
//   pub title: String,
//   pub body: String,
//   pub published: bool,
// }

// #[derive(Queryable, Insertable, Debug, Identifiable)]
#[derive(Queryable, Insertable, Debug)]
#[table_name = "merchants"]
pub struct Merchant {
  pub shop_domain: String,
  pub access_token: String,
}

// #[derive(Queryable, Insertable, Debug, Identifiable)]
// #[table_name = "charges"]
// pub struct Charge {
//   pub charge_id: String,
// }
