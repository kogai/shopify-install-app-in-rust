use crate::env::SharedState;
use crate::shopify::Shopify;
use actix_session::Session;
use actix_web::{get, http::header, web, HttpRequest, HttpResponse, Responder};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ShopifyStart {
    hmac: String,
    shop: String,
}

#[get("/shopify/start")]
pub async fn shopify_start(
    session: Session,
    info: web::Query<ShopifyStart>,
    state: web::Data<SharedState>,
    req: HttpRequest,
) -> impl Responder {
    let shopify = Shopify::new(
        state.env.app_id.clone(),
        state.env.app_secret.clone(),
        state.env.app_url.clone(),
    );
    let verified = shopify.verify_on_install(req.query_string().to_owned(), info.hmac.clone()); // .expect("Invalid hmac");
    match verified {
        Ok(()) => {
            let state: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            let request_uri = shopify.get_authorize_url(info.shop.clone(), state.clone());
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
