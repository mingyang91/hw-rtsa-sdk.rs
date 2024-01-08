#[cfg(all(test, feature = "reqwest"))]
mod test {
	use crate::manager::context::{Context, Profile};
	use crate::manager::reqwest_context::Live;
	use crate::manager::assets::{ListAssetsRequest, AssetsManager};

	#[tokio::test]
	async fn list_assets() {
		let reqwest_context = Live::new(Profile::from_env());
		let req: ListAssetsRequest = Default::default();
		let list = reqwest_context.list_assets(req).await.unwrap();
		println!("{:?}", list);
	}
}