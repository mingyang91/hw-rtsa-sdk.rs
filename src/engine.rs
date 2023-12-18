use hw_rtsa_sdk_sys::{createHRTSAEngine};
use crate::handler::HRTSAHandler;
use crate::join_param::{HRTSAScenarioType, JoinParam};
use crate::param::HRTSAParam;

pub struct HRTSAEngine {
    engine: *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEngine,
}

impl HRTSAEngine {
    pub fn new(param: HRTSAParam, handler: HRTSAHandler) -> Self {
        let mut param = param.into();
        let mut handler = handler.into();
        HRTSAEngine {
            engine: unsafe {
                createHRTSAEngine(
                    &mut param,
                    &mut handler,
                )
            },
        }
    }

    pub fn join_room(&self, room_id: &str, user_id: &str) {
        let room_id = std::ffi::CString::new(room_id).unwrap();
        let user_id = std::ffi::CString::new(user_id).unwrap();
        let mut join_param = JoinParam::new("app_id".to_string(), "token".to_string(), "user_id".to_string(), "room_id".to_string(), 0, HRTSAScenarioType::NORMAL);
        unsafe {
            ((*(*self.engine).vtable_).huawei_rtsa_IHRTSAEngine_joinRoom)(self.engine, &mut join_param.into());
        }
    }
}