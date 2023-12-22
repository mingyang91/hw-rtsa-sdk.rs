use crate::join_param::{HRTSAScenarioType, JoinParam, self};
use crate::param::{HRTSAParam, self};
use hw_rtsa_sdk_sys::{createHRTSAEngine, engine_destory, engine_joinRoom, engine_leaveRoom, huawei_rtsa_ProxyHandler_FFIAgent};
use thiserror::Error;

pub struct HRTSAEngine {
    _handler: huawei_rtsa_ProxyHandler_FFIAgent,
    engine: *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEngine,
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("param error")]
    Param(#[from] param::ConvertError),
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("param error")]
    Param(#[from] join_param::ConvertError),
    #[error("join room error (code: {code})")]
    ErrorCode { code: i32 }
}

impl HRTSAEngine {
    pub fn new(param: HRTSAParam, handler: huawei_rtsa_ProxyHandler_FFIAgent) -> Result<Self, CreateError> {
        let mut param = param.try_into()?;
        let handler_ptr = handler.raw_ptr() as *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler;
        Ok(HRTSAEngine {
            _handler: handler,
            engine: unsafe { createHRTSAEngine(&mut param, handler_ptr) },
        })
    }

    pub fn join_room(&self, room_id: &str, user_id: &str) -> Result<(), EngineError> {
        let join_param = JoinParam::new(
            "app_id".to_string(),
            "token".to_string(),
            "user_id".to_string(),
            "room_id".to_string(),
            0,
            HRTSAScenarioType::NORMAL,
        ).try_into()?;
        unsafe {
            let ret = engine_joinRoom(self.engine, &join_param);
            if ret == 0 {
                Ok(())
            } else {
                Err(EngineError::ErrorCode { code: ret })
            }
        }
    }

    pub fn leave_room(&self) -> Result<(), EngineError> {
        unsafe {
            let ret = engine_leaveRoom(self.engine);
            if ret == 0  {
                Ok(())
            } else {
                Err(EngineError::ErrorCode { code: ret })
            }
        }
    }
}

impl Drop for HRTSAEngine {
    fn drop(&mut self) {
        unsafe {
            println!("drop engine");
            // ((*(*self.engine).vtable_).huawei_rtsa_IHRTSAEngine_destory)(self.engine);
            engine_destory(self.engine);
            println!("drop done");
        }
    }
}