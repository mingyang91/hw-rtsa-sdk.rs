use serde::{Serialize, Deserialize};
use async_trait::async_trait;

// POST https://metastudio.cn-north-4.myhuaweicloud.com/v1/70b76xxxxxx34253880af501cdxxxxxx/smart-live-rooms

// {
//   "room_name" : "大自然的传说",
//   "room_description" : "课件",
//   "scene_scripts" : [ {
//     "script_name" : "大自然的传说 一",
//     "model_asset_id" : "a5d295cdb345c11bd9f36bc22ced3a7a",
//     "voice_config" : {
//       "voice_asset_id" : "a5d295cdb345c11bd9f36bc22ced3a7a"
//     },
//     "background_config" : [ {
//       "background_type" : "IMAGE",
//       "background_config" : "978f893e1de4553c183b7a805e6290f5"
//     } ],
//     "shoot_scripts" : [ {
//       "sequence_no" : 1,
//       "text_config" : {
//         "text" : "大家好，我是云笙"
//       }
//     } ]
//   } ],
//   "live_event_callback_config" : {
//     "live_event_type_callback_url" : "https://xxx/xxx/xxx?xxx=xxx",
//     "auth_type" : "NONE",
//     "callback_event_type" : [ "SHOOT_SCRIPT_SWITCH" ]
//   }
// }

enum Error {
	Reqwest(reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceConfig {
	voice_asset_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundConfig {
	background_type: String,
	background_config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextConfig {
	text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShootScript {
	sequence_no: u32,
	text_config: TextConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveEventCallbackConfig {
	live_event_type_callback_url: String,
	auth_type: String,
	callback_event_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneScript {
	script_name: String,
	model_asset_id: String,
	voice_config: VoiceConfig,
	background_config: Vec<BackgroundConfig>,
	shoot_scripts: Vec<ShootScript>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomCreateParam {
	room_name: String,
	room_description: String,
	scene_scripts: Vec<SceneScript>,
	live_event_callback_config: LiveEventCallbackConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {}

#[async_trait]
trait RoomManager {
	async fn create_room(&self, room: RoomCreateParam) -> Result<Room, Error>;
}