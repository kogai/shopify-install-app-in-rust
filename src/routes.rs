use super::models::Merchant;
use actix_session::Session;
use actix_web::{
    client::Client, get, http::header, post, web, HttpRequest, HttpResponse, Responder,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use graphql_client::GraphQLQuery;
use hmac::{crypto_mac::MacError, Hmac, Mac, NewMac};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use url::Url;

const APP_ID: &str = env!("SHOPIFY_APP_ID");
const APP_SECRET: &str = env!("SHOPIFY_APP_SECRET");
const APP_URL: &str = env!("APP_URL");

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

type HmacSha256 = Hmac<Sha256>;

fn verify_on_install(raw_message: String, expect_hmac: String) -> Result<(), MacError> {
    let parsed = querystring::querify(&raw_message).to_vec();
    let mut filtered: querystring::QueryParams = parsed
        .iter()
        .filter(|x| match x {
            ("hmac", _) => false,
            _y => true,
        })
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect();
    filtered.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut msg = querystring::stringify(filtered);
    msg.pop();

    let mut mac =
        HmacSha256::new_varkey(APP_SECRET.as_bytes()).expect("HMAC can take key of any size");
    mac.update(msg.as_bytes());
    let expect = hex::decode(expect_hmac).expect("HMAC should be decode as hex");
    mac.verify(&expect)
}

#[test]
fn test_verify_on_install_base() {
    let raw_message = "hmac=e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29&shop=producthub-sandbox.myshopify.com&timestamp=1611725114";
    let expect_hmac = "e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29";
    let actual = verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
}

#[test]
fn test_verify_on_install_sort() {
    let raw_message = "timestamp=1611725114&shop=producthub-sandbox.myshopify.com&hmac=e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29";
    let expect_hmac = "e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29";
    let actual = verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
}

#[test]
fn test_verify_on_install_done() {
    let raw_message = "code=307a46df209c323fe2dd51fd7b7d8259&hmac=8292e6e7fe425e9fa95b51fb220625559ec04b527e8485fabe48735e3f73b57d&shop=producthub-sandbox.myshopify.com&state=DP1zm3gbnhp3zjMnSa58PwT4qJYUWaQG&timestamp=1611719277";
    let expect_hmac = "8292e6e7fe425e9fa95b51fb220625559ec04b527e8485fabe48735e3f73b57d";
    let actual = verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
}

#[derive(Deserialize, Clone)]
pub struct ShopifyStart {
    hmac: String,
    shop: String,
}

fn get_authorize_url(shop_domain: String, state: String) -> String {
    let redirect_uri = format!("{}/shopify/done", APP_URL);
    let tuples = vec![
        ("client_id", APP_ID),
        ("redirect_uri", &redirect_uri),
        ("scope", "read_products"),
        ("state", &state),
    ];
    Url::parse_with_params(
        &format!("https://{}/admin/oauth/authorize", shop_domain),
        tuples,
    )
    .expect("shopDomain should be valid")
    .into_string()
}

#[get("/shopify/start")]
pub async fn shopify_start(
    session: Session,
    info: web::Query<ShopifyStart>,
    req: HttpRequest,
) -> impl Responder {
    let verified = verify_on_install(req.query_string().to_owned(), info.hmac.clone()); // .expect("Invalid hmac");
    match verified {
        Ok(()) => {
            let state: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            let request_uri = get_authorize_url(info.shop.clone(), state.clone());
            session.set("nonce", state).unwrap();
            HttpResponse::TemporaryRedirect()
                .header(header::LOCATION, request_uri)
                .finish()
        }
        Err(err) => unreachable!(
            "Invalid hmac, got=[{}], expected=[{}] {}",
            req.query_string().to_owned(),
            info.hmac.clone(),
            err
        ),
    }
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
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let verified = verify_on_install(req.query_string().to_owned(), info.hmac.clone()); // .expect("Invalid hmac");
    let nonce = (session.get::<String>("nonce").unwrap()).unwrap_or_else(|| "".to_owned());
    match verified {
        Ok(()) if nonce == info.state => {
            let body = ShopifyAccessTokenReq {
                client_id: APP_ID.to_owned(),
                client_secret: APP_SECRET.to_owned(),
                code: info.code.clone(),
            };
            let client = Client::default();
            let response = client
                .post(&format!("https://{}/admin/oauth/access_token", info.shop))
                .send_json(&body)
                .await
                .unwrap()
                .json::<ShopifyAccessTokenRes>()
                .await
                .unwrap();
            let pooled_conn = pool.get().expect("couldn't get db connection from pool");
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

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
