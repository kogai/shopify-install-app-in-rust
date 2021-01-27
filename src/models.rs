use super::schema::{charges, merchants};
use diesel::pg::upsert::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug, Identifiable)]
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
    diesel::insert_into(merchants::table)
      .values(&instance)
      .on_conflict(on_constraint("shop_domain"))
      .do_nothing()
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
