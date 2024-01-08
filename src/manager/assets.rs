use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use thiserror::Error;
use crate::manager::context::Context;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListAssetsRequestAssetSourceEnum {
	SYSTEM,
	CUSTOMIZATION,
	ALL,
}

// export class ListAssetsRequest {
// 	private 'Authorization'?: string;
// 	private 'X-Sdk-Date'?: string;
// 	private 'X-App-UserId'?: string;
// 	public limit?: number;
// 	public offset?: number;
// 	public name?: string;
// 	public tag?: string;
// 	private 'start_time'?: string;
// 	private 'end_time'?: string;
// 	private 'asset_type'?: string;
// 	private 'sort_key'?: string;
// 	private 'sort_dir'?: string;
// 	private 'asset_source'?: ListAssetsRequestAssetSourceEnum | string;
// 	private 'asset_state'?: string;
// 	private 'style_id'?: string;
// 	private 'render_engine'?: string;
// 	public sex?: string;
// 	public language?: string;
// 	private 'system_property'?: string;
// 	private 'action_editable'?: boolean;
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAssetsRequest {
	pub limit: Option<u32>,
	pub offset: Option<u32>,
	pub name: Option<String>,
	pub tag: Option<String>,
	pub sex: Option<String>,
	pub language: Option<String>,
}

impl Default for ListAssetsRequest {
	fn default() -> Self {
		ListAssetsRequest {
			limit: None,
			offset: None,
			name: None,
			tag: None,
			sex: None,
			language: None,
		}
	}
}

/**
    * @export
    * @enum {string}
    */
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DigitalAssetInfoAssetTypeEnum {
	HUMAN_MODEL,
	VOICE_MODEL,
	SCENE,
	ANIMATION,
	VIDEO,
	IMAGE,
	PPT,
	MATERIAL,
	NORMAL_MODEL,
	COMMON_FILE,
	HUMAN_MODEL_2D,
	BUSINESS_CARD_TEMPLET,
	MUSIC,
	AUDIO
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DigitalAssetInfoAssetStateEnum {
	CREATING,
	FAILED,
	UNACTIVED,
	ACTIVED,
	DELETING,
	DELETED,
	BLOCK
}
	/**
			* @export
			* @enum {string}
			*/
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DigitalAssetInfoFailTypeEnum {
	AUTOMATIC_REVIEW_REJECT,
	MANUAL_REVIEW_REJECT
}

// private 'asset_id'?: string;
// private 'asset_name'?: string;
// private 'asset_description'?: string;
// private 'create_time'?: string;
// private 'update_time'?: string;
// private 'asset_type'?: DigitalAssetInfoAssetTypeEnum | string;
// private 'asset_state'?: DigitalAssetInfoAssetStateEnum | string;
// private 'fail_type'?: DigitalAssetInfoFailTypeEnum | string;
// public reason?: string;
// private 'is_need_generate_cover'?: boolean;
// public tags?: Array<string>;
// private 'asset_extra_meta'?: AssetExtraMeta;
// private 'system_properties'?: Array<SystemProperty>;
// public files?: Array<AssetFileInfo>;
#[derive(Debug, Serialize, Deserialize)]
pub struct DigitalAssetInfo {
	pub asset_id: Option<String>,
	pub asset_name: Option<String>,
	pub asset_description: Option<String>,
	pub create_time: Option<String>,
	pub update_time: Option<String>,
	pub asset_type: Option<DigitalAssetInfoAssetTypeEnum>,
	pub asset_state: Option<DigitalAssetInfoAssetStateEnum>,
	pub fail_type: Option<DigitalAssetInfoFailTypeEnum>,
	pub reason: Option<String>,
	pub is_need_generate_cover: Option<bool>,
	pub tags: Option<Vec<String>>,
	// pub asset_extra_meta: AssetExtraMeta,
	// pub system_properties: Vec<SystemProperty>,
	// pub files: Vec<AssetFileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DigitalAssetsList {
	pub assets: Vec<DigitalAssetInfo>,
	pub count: u32,
}

#[async_trait]
pub trait AssetsManager<C: Context> {
	async fn list_assets(&self, req: ListAssetsRequest) -> Result<DigitalAssetsList, C::Error<ListAssetsError>>;
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ListAssetsError {
	#[error("invalid parameter")]
	InvalidParameter { error_code: String, error_msg: String },
}


#[async_trait]
impl <C: Context> AssetsManager<C> for C {
	async fn list_assets(&self, req: ListAssetsRequest) -> Result<DigitalAssetsList, C::Error<ListAssetsError>> {
		self.execute("GET", "/v1/f913be1782174fccbb2dfa6bf61dac2c/digital-assets", req).await
	}
}