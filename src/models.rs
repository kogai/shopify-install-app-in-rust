use super::schema::merchants;

#[derive(Queryable, Insertable, Debug,Identifiable)]
#[primary_key(shop_domain)]
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
