use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use app::{env::SharedState, routes};
use std::env;

pub const DATABASE_URL: &str = env!("DATABASE_URL");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let state = SharedState::default;

    HttpServer::new(move || {
        App::new()
            .data(state)
            // TODO: Set key and force secure.
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(routes::shopify_start::shopify_start)
            .service(routes::shopify_done::shopify_done)
            .service(routes::echo::echo)
            .route("/hey", web::get().to(routes::manual_hello::manual_hello))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
