use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use app::routes;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub const DATABASE_URL: &str = env!("DATABASE_URL");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool = Pool::builder().build(manager).expect("");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
