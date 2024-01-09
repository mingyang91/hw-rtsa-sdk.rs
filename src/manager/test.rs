#[cfg(all(test, feature = "reqwest"))]
mod test {
	use crate::manager::context::{Context, Profile};
	use crate::manager::reqwest_context::Live;
	use crate::manager::assets::{ListAssetsRequest, AssetsManager};
	use crate::manager::cslr::CreateSmartLiveRoomReq;
	use crate::manager::cslr::CreateSmartLiveRoomRsp;
	use crate::manager::room::{RoomManager};

	#[tokio::test]
	async fn list_assets() {
		let reqwest_context = Live::new(Profile::from_env());
		let req: ListAssetsRequest = Default::default();
		let list = reqwest_context.list_assets(req).await.unwrap();
		println!("{:?}", list);
	}

	// {
	// 	"room_name" : "大自然的传说",
	// 	"room_description" : "课件",
	// 	"scene_scripts" : [ {
	// 		"script_name" : "大自然的传说 一",
	// 		"model_asset_id" : "a5d295cdb345c11bd9f36bc22ced3a7a",
	// 		"voice_config" : {
	// 			"voice_asset_id" : "a5d295cdb345c11bd9f36bc22ced3a7a"
	// 		},
	// 		"background_config" : [ {
	// 			"background_type" : "IMAGE",
	// 			"background_config" : "978f893e1de4553c183b7a805e6290f5"
	// 		} ],
	// 		"shoot_scripts" : [ {
	// 			"sequence_no" : 1,
	// 			"text_config" : {
	// 				"text" : "大家好，我是云笙"
	// 			}
	// 		} ]
	// 	} ],
	// 	"live_event_callback_config" : {
	// 		"live_event_type_callback_url" : "https://xxx/xxx/xxx?xxx=xxx",
	// 		"auth_type" : "NONE",
	// 		"callback_event_type" : [ "SHOOT_SCRIPT_SWITCH" ]
	// 	}
	// }
	#[tokio::test]
	async fn create_room() {
		let reqwest_context = Live::new(Profile::from_env());
		let req = CreateSmartLiveRoomReq::builder()
		.room_name("大自然的传说")
		.try_into().expect("CreateSmartLiveRoomReq");
		let room = reqwest_context.create_room(req).await.unwrap();
		println!("{:?}", room);
	}
}