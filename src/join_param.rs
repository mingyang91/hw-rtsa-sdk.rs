use std::mem::transmute;
use hw_rtsa_sdk_sys::huawei_rtsa_HRTSAJoinParam;
use thiserror::Error;

use crate::utils::ToFixedBytes;

pub struct JoinParam {
    pub app_id: String,
    pub token: String,
    pub user_id: String,
    pub room_id: String,
    pub ctime: i64,
    pub scenario: HRTSAScenarioType,
}

/* *
 * 加入房间时的参数：场景
 * 0:小会场（默认）
 * 1:大会场（千人-RTSA不支持）
 * 2:p2p（RTSA不支持）
 * 3: 畅连专用
 */
pub enum HRTSAScenarioType {
    NORMAL = 0,
    LARGE = 1,
    P2P = 2,
    AutoCmd = 3
}

impl JoinParam {
    pub fn new(app_id: String, token: String, user_id: String, room_id: String, ctime: i64, scenario: HRTSAScenarioType) -> Self {
        JoinParam {
            app_id,
            token,
            user_id,
            room_id,
            ctime,
            scenario,
        }
    }
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("string `{str}` to long to field `{name}[{max_size}]`")]
    StringToLong { name: String, max_size: usize, str: String }
}

impl TryInto<huawei_rtsa_HRTSAJoinParam> for JoinParam {
    type Error = ConvertError;
    fn try_into(self) -> Result<huawei_rtsa_HRTSAJoinParam, Self::Error> {
        let app_id: [u8; 129] = self.app_id.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "app_id".to_string(), max_size: 129, str: self.app_id })?;
        let token: [u8; 2049] = self.token.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "token".to_string(), max_size: 2049, str: self.token })?;
        let user_id: [u8; 65] = self.user_id.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "user_id".to_string(), max_size: 65, str: self.user_id })?;
        let room_id: [u8; 65] = self.room_id.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "room_id".to_string(), max_size: 65, str: self.room_id })?;


        unsafe {
            let param = huawei_rtsa_HRTSAJoinParam {
                appId: transmute(app_id),
                token: transmute(token),
                userId: transmute(user_id),
                roomId: transmute(room_id),
                ctime: self.ctime,
                scenario: self.scenario as u32,
            };
            Ok(param)
        }
    }
}