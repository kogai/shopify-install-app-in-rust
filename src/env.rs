use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub const APP_ID: &str = env!("SHOPIFY_APP_ID");
pub const APP_SECRET: &str = env!("SHOPIFY_APP_SECRET");
pub const APP_URL: &str = env!("APP_URL");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
struct Env {
  app_id: String,
  app_secret: String,
  app_url: String,
}

impl Env {
  fn new(app_id: String, app_secret: String, app_url: String) -> Self {
    Env {
      app_id,
      app_secret,
      app_url,
    }
  }
}

#[derive(Clone)]
pub struct SharedState {
  conn: DbPool,
  env: Env,
}

impl Default for SharedState {
  fn default() -> Self {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");

    let env = Env::new(
      env!("SHOPIFY_APP_ID").to_owned(),
      env!("SHOPIFY_APP_SECRET").to_owned(),
      env!("APP_URL").to_owned(),
    );

    let manager = ConnectionManager::<PgConnection>::new(env!("DATABASE_URL"));
    let conn = Pool::builder().build(manager).expect("");

    SharedState { env, conn }
  }
}
