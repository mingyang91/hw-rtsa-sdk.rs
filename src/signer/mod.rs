use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex;
use chrono;

const ALGORITHM: &str = "SDK-HMAC-SHA256";
const HEADER_X_DATE: &str = "X-Sdk-Date";
const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_SHA256: &str = "x-sdk-content-sha256";

pub struct Signer {
  key: String,
  secret: String,
}

pub trait SignableRequest
where {
  type Headers: Iterator<Item = (String, String)>;
  type Body: AsRef<[u8]>;
  fn host(&self) -> &str;
  fn method(&self) -> &str;
  fn uri(&self) -> &str;
  fn headers(&self) -> Self::Headers;
  fn header(&self, key: &str) -> &str;
  fn set_header(&mut self, key: String, value: String);
  fn query(&self) -> &str;
  fn body(&self) -> &Self::Body;
}

impl Signer {
  pub fn new(key: &str, secret: &str) -> Self {
    Signer {
      key: key.to_string(),
      secret: secret.to_string(),
    }
  }

  pub fn sign<Req: SignableRequest>(&self, req: &mut Req) {
    let header_time = req.header(HEADER_X_DATE).to_string();
    if header_time.is_empty() {
      let header_time = chrono::Utc::now().to_rfc2822();
      req.set_header(HEADER_X_DATE.to_string(), header_time.to_string());
    }

    let mut signed_headers: Vec<(String, String)> = req.headers()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    signed_headers.sort_by(|a, b| a.0.cmp(&b.0));

    let canonical_request = self.canonical_request(req, &signed_headers);
    let string_to_sign = Self::string_to_sign(&canonical_request, &header_time);
    let signature = self.sign_string_to_sign(&string_to_sign);
    req.set_header(HEADER_AUTHORIZATION.to_string(), self.auth_header_value(&signature, &signed_headers));
  }

  fn canonical_headers(headers: &Vec<(String, String)>) -> String {
    let mut canonical_headers: Vec<String> = Vec::new();
    for (key, value) in headers {
      canonical_headers.push(format!("{}:{}\n", key.to_lowercase(), value.trim()));
    }
    canonical_headers.join("")
  }

  fn canonical_request<Req: SignableRequest>(
    &self, 
    req: &mut Req,
    signed_headers: &Vec<(String, String)>,
  ) -> String {
    let content_sha256 = Self::content_sha256(req.body().as_ref());
    req.set_header(HEADER_CONTENT_SHA256.to_string(), content_sha256.clone());
    let canonical_request = vec![
      req.method().to_string(),
      req.uri().to_string(),
      req.query().to_string(),
      Self::canonical_headers(signed_headers),
      signed_headers.iter().map(|(key, _)| key.to_lowercase()).collect::<Vec<_>>().join(";"),
      content_sha256
    ];
    canonical_request.join("\n")
  }

  fn content_sha256(body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    hex::encode(hasher.finalize())
  }

  fn string_to_sign(canonical_request: &str, header_time: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(canonical_request.as_bytes());
    let hashed_canonical_request = hex::encode(hasher.finalize());
    let string_to_sign = vec![
      ALGORITHM.to_string(),
      header_time.to_string(),
      hashed_canonical_request
    ];
    string_to_sign.join("\n")
  }

  fn sign_string_to_sign(&self, string_to_sign: &str) -> String {
    self.hmac_sha256(self.secret.as_bytes(), string_to_sign.as_bytes())
  }

  fn auth_header_value(&self, signature: &str, signed_headers: &Vec<(String, String)>) -> String {
    let signed_headers = signed_headers.iter().map(|(key, _)| key.to_lowercase()).collect::<Vec<_>>().join(";");
    let auth_header_value = vec![
      format!("Access={}", self.key),
      format!("SignedHeaders={}", signed_headers),
      format!("Signature={}", signature)
    ];
    ALGORITHM.to_string() + " " + &auth_header_value.join(", ")
  }

  fn hmac_sha256(&self, key: &[u8], message: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
      .expect("HMAC can take key of any size");
    mac.update(message);
    let result = mac.finalize();

    hex::encode(result.into_bytes())
  }
}

use std::str::FromStr;
#[cfg(feature = "http")]
use http::{header::HeaderName, HeaderValue, request::Request};
#[cfg(feature = "http")]
impl <T> SignableRequest for Request<T>
where T: AsRef<[u8]> {
  type Headers = std::vec::IntoIter<(String, String)>;
  type Body = T;
  fn host(&self) -> &str {
    let Some(host) = self.uri().host() else {
      return "";
    };

    return host;
  }

  fn method(&self) -> &str {
    self.method().as_str()
  }

  fn uri(&self) -> &str {
    self.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
  }

  fn header(&self, key: &str) -> &str {
    let Some(value) = self.headers().get(key) else {
      return "";
    };

    value.to_str().unwrap_or("")
  }

  fn headers(&self) -> Self::Headers {
    self.headers()
      .into_iter()
      .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
      .collect::<Vec<_>>()
      .into_iter()
  }

  fn set_header(&mut self, key: String, value: String) {
    self.headers_mut()
      .insert(
        HeaderName::from_str(&key).expect("set header error"),
        HeaderValue::from_str(&value).expect("set header error")
      );
  }

  fn query(&self) -> &str {
    self.uri().query().unwrap_or("")
  }

  fn body(&self) -> &Self::Body {
    self.body()
  }
}

#[cfg(test)]
mod test {
  use http::request::Builder;
  use http::request::Request;
  use crate::signer::Signer;

  #[derive(Debug)]
  struct Empty {}
  impl AsRef<[u8]> for Empty {
    fn as_ref(&self) -> &[u8] {
      &[]
    }
  }
  
  #[test]
  fn function1() {
    // var r = new signer.HttpRequest("GET", "service.region.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1");
    let mut request = Request::builder()
      .method("GET")
      .uri("http://service.region.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .body(Empty {})
      .expect("request builder error");

    let signer = Signer::new("ak", "sk");
    signer.sign(&mut request);
    println!("{:?}", request);
  }
}