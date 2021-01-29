use crate::env::{SharedState, APP_ID, APP_SECRET};
use crate::models::Merchant;
use crate::shopify::Shopify;
use actix_session::Session;
use actix_web::{client::Client, get, web, HttpRequest, HttpResponse, Responder};
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "gql/schema.json",
  query_path = "gql/charge_query.graphql"
)]

pub struct ChargeQuery;

pub fn charge() {
  ChargeQuery::build_query(charge_query::Variables {
    charge_id: "".to_owned(),
  });
}

#[derive(Deserialize, Clone)]
pub struct ShopifyStart {
  hmac: String,
  shop: String,
}

#[derive(Deserialize, Clone)]
pub struct ShopifyDone {
  shop: String,
  code: String,
  hmac: String,
  state: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShopifyAccessTokenReq {
  client_id: String,
  client_secret: String,
  code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShopifyAccessTokenRes {
  access_token: String,
  scope: String,
}

#[get("/shopify/done")]
pub async fn shopify_done(
  session: Session,
  info: web::Query<ShopifyDone>,
  state: web::Data<SharedState>,
  req: HttpRequest,
) -> impl Responder {
  let shopify = Shopify::new(
    state.env.app_id.clone(),
    state.env.app_secret.clone(),
    state.env.app_url.clone(),
  );
  let verified = shopify.verify_on_install(req.query_string().to_owned(), info.hmac.clone()); // .expect("Invalid hmac");
  let nonce = (session.get::<String>("nonce").unwrap()).unwrap_or_else(|| "".to_owned());
  match verified {
    Ok(()) if nonce == info.state => {
      let body = ShopifyAccessTokenReq {
        client_id: APP_ID.to_owned(),
        client_secret: APP_SECRET.to_owned(),
        code: info.code.clone(),
      };
      // TODO: Extract to shopify.rs
      let client = Client::default();
      let response = client
        .post(&format!("https://{}/admin/oauth/access_token", info.shop))
        .send_json(&body)
        .await
        .unwrap()
        .json::<ShopifyAccessTokenRes>()
        .await
        .unwrap();
      let pooled_conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");
      let m = Merchant::new(&pooled_conn, info.shop.clone(), response.access_token);
      println!("Saved merchant: {:?} {:?}", m.shop_domain, m.access_token);
      HttpResponse::Ok().body("Hello world!")
    }
    Ok(()) => unreachable!("Invalid nonce, expected=[{}] got=[{}]", nonce, info.state),
    Err(err) => unreachable!(
      "Invalid hmac, got=[{}], expected=[{}] {}",
      req.query_string().to_owned(),
      info.hmac.clone(),
      err
    ),
  }
}
