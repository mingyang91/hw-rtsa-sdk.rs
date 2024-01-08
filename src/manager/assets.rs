use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::manager::context::{Context, Error};
use thiserror::Error;

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
	pub asset_id: String,
	pub asset_name: String,
	pub asset_description: String,
	pub create_time: String,
	pub update_time: String,
	pub asset_type: DigitalAssetInfoAssetTypeEnum,
	pub asset_state: DigitalAssetInfoAssetStateEnum,
	pub fail_type: DigitalAssetInfoFailTypeEnum,
	pub reason: String,
	pub is_need_generate_cover: bool,
	pub tags: Vec<String>,
	// pub asset_extra_meta: AssetExtraMeta,
	// pub system_properties: Vec<SystemProperty>,
	// pub files: Vec<AssetFileInfo>,
}

#[async_trait]
trait AssetsManager {
	async fn list_assets(&self, req: ListAssetsRequest) -> Result<Vec<DigitalAssetInfo>, Error<ListAssetsError>>;
}

#[derive(Debug, Serialize, Deserialize, Error)]
enum ListAssetsError {

}


#[async_trait]
impl <C: Context> AssetsManager for C {
	async fn list_assets(&self, req: ListAssetsRequest) -> Result<Vec<DigitalAssetInfo>, Error<ListAssetsError>> {
		todo!()
	}
}