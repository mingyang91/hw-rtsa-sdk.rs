use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex;
use chrono;

const ALGORITHM: &str = "SDK-HMAC-SHA256";
const HEADER_X_DATE: &str = "X-Sdk-Date";
const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_SHA256: &str = "x-sdk-content-sha256";

pub trait Clock {
  fn now() -> chrono::DateTime<chrono::Utc>;
}

pub struct IO {}
impl Clock for IO {
  fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
  }
}

pub struct Signer<C: Clock> {
  key: String,
  secret: String,
  _context: std::marker::PhantomData<C>,
}

pub trait SignableRequest
where {
  type Headers: Iterator<Item = (String, String)>;
  type Body: AsRef<[u8]>;
  fn host(&self) -> &str;
  fn method(&self) -> &str;
  fn uri(&self) -> &str;
  fn path(&self) -> &str;
  fn headers(&self) -> Self::Headers;
  fn header(&self, key: &str) -> &str;
  fn set_header(&mut self, key: String, value: String);
  fn query(&self) -> &str;
  fn body(&self) -> &Self::Body;
  fn sign_with<C: Clock>(&mut self, signer: &Signer<C>) 
  where Self: Sized {
    signer.sign(self);
  }
}

impl <C: Clock> Signer<C> {
  pub fn new(key: &str, secret: &str) -> Self {
    Signer {
      key: key.to_string(),
      secret: secret.to_string(),
      _context: std::marker::PhantomData,
    }
  }

  pub fn sign<Req: SignableRequest>(&self, req: &mut Req) {
    let mut header_time = req.header(HEADER_X_DATE).to_string();
    if header_time.is_empty() {
      header_time = C::now().format("%Y%m%dT%H%M%SZ").to_string();
      req.set_header(HEADER_X_DATE.to_string(), header_time.to_string());
    }

    if req.header("Host").is_empty() {
      req.set_header("Host".to_string(), req.host().to_string());
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
    let content = req.body().as_ref();
    let content_sha256 = Self::content_sha256(content);
    let mut path = req.path().to_string();
    if !path.ends_with('/') {
      path.push('/');
    }
    let canonical_request = vec![
      req.method().to_string(),
      path,
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

  fn path(&self) -> &str {
    self.uri().path()
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
  use serde::Serialize;
  use http::request::Builder;
  use http::request::Request;
  use crate::signer::{Signer, SignableRequest, Clock};

  #[derive(Debug)]
  struct Empty {}
  impl AsRef<[u8]> for Empty {
    fn as_ref(&self) -> &[u8] {
      &[]
    }
  }

  struct Matrix {}
  impl Clock for Matrix {
    fn now() -> chrono::DateTime<chrono::Utc> {
      // 20120101T000000Z
      chrono::DateTime::parse_from_rfc3339("2012-01-01T00:00:00Z").unwrap().into()
    }
  }
  
  #[test]
  fn sign_get() {
    let mut request = Request::builder()
      .method("GET")
      .uri("http://endpoint.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .header("Content-Type", "application/json")
      .body(Empty {})
      .expect("request builder error");

    let signer = Signer::<Matrix>::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");
    request.sign_with(&signer);
    let authorization = "SDK-HMAC-SHA256 Access=QTWAOYTTINDUT2QVKYUC, SignedHeaders=content-type;host;x-sdk-date, Signature=486fb116518ebda891aa35f99750ab3fc8f1fa22315e3ba619b016a2261503f4";
    assert_eq!(request.header("Authorization"), authorization);
  }

  #[derive(Serialize)]
  struct Info {
    foo: String,
    bar: i32,
    baz: bool,
  }

  impl AsRef<[u8]> for Info {
    fn as_ref(&self) -> &[u8] {
      serde_json::to_string(self).unwrap().as_bytes()
    }
  }

  #[test]
  fn sign_post() {
    let mut request = Request::builder()
      .method("POST")
      .uri("http://endpoint.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .header("Content-Type", "application/json")
      .body(Info { foo: "foo".to_string(), bar: 114514, baz: true })
      .expect("request builder error");

    let signer = Signer::<Matrix>::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");
    request.sign_with(&signer);
    let authorization = "SDK-HMAC-SHA256 Access=QTWAOYTTINDUT2QVKYUC, SignedHeaders=content-type;host;x-sdk-date, Signature=486fb116518ebda891aa35f99750ab3fc8f1fa22315e3ba619b016a2261503f4";
    assert_eq!(request.header("Authorization"), authorization);
  }

  #[test]
  fn real_world() {
    let mut request = Request::builder()
      .method("POST")
      .uri("http://endpoint.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .header("Content-Type", "application/json")
      .body(Info { foo: "foo".to_string(), bar: 114514, baz: true })
      .expect("request builder error");

    let signer = Signer::<Matrix>::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");
    request.sign_with(&signer);
    send(request);
  }
}