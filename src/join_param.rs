use std::mem::{transmute};
use hw_rtsa_sdk_sys::huawei_rtsa_HRTSAJoinParam;

//    pub appId: [::std::os::raw::c_char; 129usize],
//     pub token: [::std::os::raw::c_char; 2049usize],
//     pub userId: [::std::os::raw::c_char; 65usize],
//     pub roomId: [::std::os::raw::c_char; 65usize],
//     pub ctime: ::std::os::raw::c_longlong,
//     pub scenario: huawei_rtsa_HRTSAScenarioType,
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
    AUTO_CMD = 3
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

impl Into<huawei_rtsa_HRTSAJoinParam> for JoinParam {
    fn into(self) -> huawei_rtsa_HRTSAJoinParam {
        let mut app_id: [u8; 129] = [0; 129];
        app_id.copy_from_slice(self.app_id.as_bytes());
        let mut token: [u8; 2049] = [0; 2049];
        token.copy_from_slice(self.token.as_bytes());
        let mut user_id: [u8; 65] = [0; 65];
        user_id.copy_from_slice(self.user_id.as_bytes());
        let mut room_id: [u8; 65] = [0; 65];
        room_id.copy_from_slice(self.room_id.as_bytes());

        unsafe {
            huawei_rtsa_HRTSAJoinParam {
                appId: transmute(app_id),
                token: transmute(token),
                userId: transmute(user_id),
                roomId: transmute(room_id),
                ctime: self.ctime,
                scenario: self.scenario as u32,
            }
        }
    }
}