use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use std::fmt::Debug;
use std::error::Error as StdError;
use std::sync::Arc;
use std::str::FromStr;
use crate::signer::{SignableRequest, Signer, JsonBody};
#[cfg(feature = "reqwest")]
use reqwest::Method;


#[derive(thiserror::Error, Debug)]
pub enum Error<T>
where T: Debug + StdError + Serialize + Send + Sync,
      T: DeserializeOwned {
	#[cfg(feature = "reqwest")]
	#[error("reqwest error")]
	Reqwest(reqwest::Error),
	#[cfg(feature = "reqwest")]
	#[error("method error")]
	InvalidMethod(#[source] http::method::InvalidMethod),
	#[error("domain error")]
	Domain(#[from] T),
}

#[async_trait]
pub trait Context 
where Self: Send + Sync {
	fn app_user_id(&self) -> String;
	fn region(&self) -> String;
	fn signer(&self) -> &Signer;
	async fn execute<Param, Resp, DomainError>(
		&self,
		method: &str,
		path: &str,
		req: Param) -> Result<Resp, Error<DomainError>>
	where Param: Serialize + Debug + Send,
		Resp: DeserializeOwned + Debug + Send,
		DomainError: Debug + StdError + DeserializeOwned + Serialize + Send + Sync;
}

struct InnerLive {
	app_user_id: String,
	region: String,
	signer: Signer,
	#[cfg(feature = "reqwest")]
	client: reqwest::Client,
}

struct Live {
	inner: Arc<InnerLive>,
}


#[async_trait]
impl Context for Live {
	fn app_user_id(&self) -> String {
		self.inner.app_user_id.clone()
	}

	fn region(&self) -> String {
		self.inner.region.clone()
	}

	fn signer(&self) -> &Signer {
		&self.inner.signer
	}

	#[cfg(not(feature = "reqwest"))]
	async fn execute<Param, Resp, DomainError>(
		&self,
		method: &str,
		path: &str,
		req: Param) -> Result<Resp, Error<DomainError>>
	where Param: Serialize + Debug + Send,
		Resp: DeserializeOwned + Debug + Send,
		DomainError: Debug + StdError + DeserializeOwned + Serialize + Send + Sync {
		unimplemented!("feature `reqwest` is not enabled")
	}

	#[cfg(feature = "reqwest")]
	async fn execute<Param, Resp, DomainError>(
		&self,
		method: &str,
		path: &str,
		req: Param) -> Result<Resp, Error<DomainError>>
	where Param: Serialize + Debug + Send,
		Resp: DeserializeOwned + Debug + Send,
		DomainError: Debug + StdError + DeserializeOwned + Serialize + Send + Sync {
		// https://metastudio.cn-north-4.myhuaweicloud.com/v1/70b76xxxxxx34253880af501cdxxxxxx/smart-live-rooms
		let method = Method::from_str(method).map_err(Error::InvalidMethod)?;
		let url = format!("https://metastudio.{}.myhuaweicloud.com{}", self.inner.region, path);
		let req = self.inner.client.request(method, &url)
			.header("Content-Type", "application/json")
			.body(JsonBody(req))
			.build()
			.map_err(Error::Reqwest)?
			.sign_with(&self.inner.signer);
		self.inner.client.execute(req)
			.await
			.map_err(Error::Reqwest)?
			.json()
			.await
			.map_err(Error::Reqwest)
	}
}

