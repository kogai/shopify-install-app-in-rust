use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{test, App};

  #[actix_rt::test]
  async fn test_echo_post() {
    let mut app = test::init_service(App::new().service(echo)).await;
    // let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let req = test::TestRequest::post().uri("/echo").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
  }
}
