use actix_session::{CookieSession};
use actix_web::{ web, App, HttpServer};
use app::{routes};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: embed_migrations!
    // use shopify_install_app_in_rust::schema::posts::dsl::*;
    // let connection = establish_connection();
    // let _post = create_post(
    //     &connection,
    //     "my title",
    //     "this is good post i've never wrote.",
    // );
    // let results = posts
    //     .limit(5)
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");
    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // TODO: Set key and force secure.
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(routes::shopify_start)
            .service(routes::shopify_done)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
