use hmac::{crypto_mac::MacError, Hmac, Mac, NewMac};
use sha2::Sha256;
use url::Url;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct Shopify {
  pub app_id: String,
  pub app_secret: String,
  pub app_url: String,
}

impl Shopify {
  pub fn new(app_id: String, app_secret: String, app_url: String) -> Self {
    Shopify {
      app_id,
      app_secret,
      app_url,
    }
  }

  pub fn verify_on_install(
    &self,
    raw_message: String,
    expect_hmac: String,
  ) -> Result<(), MacError> {
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
      HmacSha256::new_varkey(self.app_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(msg.as_bytes());
    let expect = hex::decode(expect_hmac).expect("HMAC should be decode as hex");
    mac.verify(&expect)
  }

  pub fn get_authorize_url(&self, shop_domain: String, state: String) -> String {
    let redirect_uri = format!("{}/shopify/done", self.app_url);
    let tuples = vec![
      ("client_id", self.app_id.as_str()),
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
}

#[cfg(test)]
mod test {
  use super::*;

  fn setup() -> Shopify {
    Shopify::new(
      "app_id".to_owned(),
      "app_secret".to_owned(),
      "https://example.com".to_owned(),
    )
  }

  #[test]
  fn test_verify_on_install_base() {
    let shopify = setup();
    let raw_message = "hmac=e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29&shop=sandbox.myshopify.com&timestamp=1611725114";
    let expect_hmac = "2491ec05efe9e6fe47e65994f79e1ca2c252ecbc5b0da9e95b36877794684473";
    let actual = shopify.verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
  }

  #[test]
  fn test_verify_on_install_sort() {
    let shopify = setup();
    let raw_message = "timestamp=1611725114&shop=sandbox.myshopify.com&hmac=e83f33fcec83fced1d46c18b20ea9dc9b61f60dd0373f6d7cceb4095f72caf29";
    let expect_hmac = "2491ec05efe9e6fe47e65994f79e1ca2c252ecbc5b0da9e95b36877794684473";
    let actual = shopify.verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
  }

  #[test]
  fn test_verify_on_install_done() {
    let shopify = setup();
    let raw_message = "code=307a46df209c323fe2dd51fd7b7d8259&hmac=8292e6e7fe425e9fa95b51fb220625559ec04b527e8485fabe48735e3f73b57d&shop=sandbox.myshopify.com&state=DP1zm3gbnhp3zjMnSa58PwT4qJYUWaQG&timestamp=1611719277";
    let expect_hmac = "40ac416039b9008e0150dc8c68ea9f04ce22b8dae7f7027f5587a2cd921bb8af";
    let actual = shopify.verify_on_install(raw_message.to_owned(), expect_hmac.to_owned());
    assert_eq!(actual, Ok(()));
  }
}
