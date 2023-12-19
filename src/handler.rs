use hw_rtsa_sdk_sys::{createHandler, huawei_rtsa_ProxyHandler};

unsafe extern "C" fn on_join_room_success(this: *const huawei_rtsa_ProxyHandler,
                                          user_id: *const std::os::raw::c_char,
                                          room_id: *const std::os::raw::c_char,
                                          elapsed: std::os::raw::c_int) {
    let user_id = std::ffi::CStr::from_ptr(user_id).to_str().unwrap();
    let room_id = std::ffi::CStr::from_ptr(room_id).to_str().unwrap();
    let elapsed = elapsed as i32;
}


pub struct HRTSAHandler {
    pub(crate) handler: *mut hw_rtsa_sdk_sys::huawei_rtsa_ProxyHandler,
}

impl HRTSAHandler {
    pub fn new() -> Self {
        HRTSAHandler {
            handler: unsafe { createHandler() },
        }
    }

    pub fn raw_ptr(&self) -> *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler {
        self.handler as *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler
    }
}

