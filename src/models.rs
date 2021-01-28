use super::schema::{charges, merchants};
use diesel::debug_query;
use diesel::pg::upsert::*;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::*;

#[derive(Queryable, Insertable, Debug, Identifiable, AsChangeset)]
#[primary_key(shop_domain)]
#[table_name = "merchants"]
pub struct Merchant {
  pub shop_domain: String,
  pub access_token: String,
}

impl Merchant {
  pub fn new(conn: &PgConnection, shop_domain: String, access_token: String) -> Self {
    let instance = Merchant {
      shop_domain,
      access_token,
    };
    let query = diesel::insert_into(merchants::table)
      .values(&instance)
      .on_conflict(on_constraint("shop_domain"))
      .do_update()
      .set(&instance);

    println!("{}", debug_query::<Pg, _>(&query).to_string());

    query
      .get_result::<Merchant>(conn)
      .expect("Error saving new merchant")
  }
}

#[derive(Queryable, Insertable, Associations, Debug)]
#[belongs_to(Merchant, foreign_key = "shop_domain")]
#[table_name = "charges"]
pub struct Charge {
  pub charge_id: String,
  pub shop_domain: String,
  // TODO:
  // created_at on install
  // updated_at on install after uninstall
}
