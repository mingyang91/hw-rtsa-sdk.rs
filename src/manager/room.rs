use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::manager::context::Context;
use huawei_cloud_api_definitions_MetaStudio::CreateSmartLiveRoom::{CreateSmartLiveRoomRsp, CreateSmartLiveRoomReq};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("API Common Error")]
    CommonError {
        error_code: String,
        error_msg: String,
    },
}

#[async_trait]
pub trait RoomManager<C: Context> {
	async fn create_room(&self, req: CreateSmartLiveRoomReq) -> Result<CreateSmartLiveRoomRsp, C::Error<Error>>;
}

#[async_trait]
impl<C: Context> RoomManager<C> for C {
	async fn create_room(&self, req: CreateSmartLiveRoomReq) -> Result<CreateSmartLiveRoomRsp, C::Error<Error>> {
		self.execute("POST", "/v1/f913be1782174fccbb2dfa6bf61dac2c/smart-live-rooms", req).await
	}
}