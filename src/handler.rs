use std::collections::HashMap;
use std::ffi::{c_char, c_int};
use std::sync::{Mutex, Arc, RwLock, OnceLock};

use hw_rtsa_sdk_sys::{createHandler, huawei_rtsa_ProxyHandler, huawei_rtsa_IHRTSAEventHandler, huawei_rtsa_ProxyHandler_FFIProtocol};

// struct HandlerRegister {
//     register: RwLock<HashMap<*const huawei_rtsa_ProxyHandler, HRTSAHandler>>,
// }
// impl HandlerRegister {
//     fn get(&self, handler: &*const huawei_rtsa_ProxyHandler) -> Option<HRTSAHandler> {
//         self.register.read().expect("read lock failed").get(handler).map(|h| h.clone())
//     }
//     fn insert(&self, handler: *const huawei_rtsa_ProxyHandler, h: HRTSAHandler) {
//         self.register.write().expect("write lock failed").insert(handler, h);
//     }
//     fn remove(&self, handler: &*const huawei_rtsa_ProxyHandler) {
//         self.register.write().expect("write lock failed").remove(handler);
//     }
// }

// unsafe impl Sync for HandlerRegister {}
// unsafe impl Send for HandlerRegister {}

// impl Default for HandlerRegister {
//     fn default() -> Self {
//         HandlerRegister {
//             register: RwLock::new(HashMap::new()),
//         }
//     }
// }

// static REGISTER: OnceLock<HandlerRegister> = OnceLock::new();

// fn get_register() -> &'static HandlerRegister {
//     REGISTER.get_or_init(Default::default)
// }


// unsafe extern "C" fn on_join_room_success(this: *const huawei_rtsa_ProxyHandler,
//                                           user_id: *const c_char,
//                                           room_id: *const c_char,
//                                           elapsed: c_int) {
//     let register = get_register();
//     if let Some(handler) = register.get(&this) {
//         let mut guard = handler.inner.lock().expect("lock failed");
//         guard.on_join_room_success(user_id, room_id, elapsed);
//     } else {
//         println!("on_join_room_success: handler not found");
//     }
// }

// unsafe extern "C" fn on_error(this: *const huawei_rtsa_ProxyHandler,
//                               error_code: c_int,
//                               error_msg: *const c_char) {
//     let register = get_register();
//     if let Some(handler) = register.get(&this) {
//         let mut guard = handler.inner.lock().expect("lock failed");
//         guard.on_error(error_code, error_msg);
//     } else {
//         println!("on_error: handler not found");
//     }
// }

// unsafe extern "C" fn on_warning(this: *const huawei_rtsa_ProxyHandler,
//                                 warning_code: c_int,
//                                 warning_msg: *const c_char) {
//     let register = get_register();
//     if let Some(handler) = register.get(&this) {
//         let mut guard = handler.inner.lock().expect("lock failed");
//         guard.on_warning(warning_code, warning_msg);
//     } else {
//         println!("on_warning: handler not found");
//     }
// }

pub struct HRTSAHandlerInner {
    handler: *mut hw_rtsa_sdk_sys::huawei_rtsa_ProxyHandler,
}

impl HRTSAHandlerInner {
    pub fn new() -> Self {
        let handler = unsafe {
            let handler = createHandler();
            handler
        };
        HRTSAHandlerInner {
            handler,
        }
    }
}

// pub trait Handler {
//     fn raw_ptr(&self) -> *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler;

//     fn on_join_room_success(&mut self, user_id: *const c_char, room_id: *const c_char, elapsed: c_int) {
//         let user_id = unsafe { std::ffi::CStr::from_ptr(user_id).to_str().unwrap() };
//         let room_id = unsafe { std::ffi::CStr::from_ptr(room_id).to_str().unwrap() };
//         println!("on_join_room_success: user_id={}, room_id={}, elapsed={}", user_id, room_id, elapsed);
//     }

//     fn on_error(&mut self, error_code: c_int, error_msg: *const c_char) {
//         let error_msg = unsafe { std::ffi::CStr::from_ptr(error_msg).to_str().unwrap() };
//         println!("on_error: error_code={}, error_msg={}", error_code, error_msg);
//     }

//     fn on_warning(&mut self, warning_code: c_int, warning_msg: *const c_char) {
//         let warning_msg = unsafe { std::ffi::CStr::from_ptr(warning_msg).to_str().unwrap() };
//         println!("on_warning: warning_code={}, warning_msg={}", warning_code, warning_msg);
//     }
// }

impl huawei_rtsa_ProxyHandler_FFIProtocol for HRTSAHandlerInner {
    fn raw_ptr(&self) -> *mut hw_rtsa_sdk_sys::huawei_rtsa_ProxyHandler {
        self.handler as *mut hw_rtsa_sdk_sys::huawei_rtsa_ProxyHandler
    }

    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

// #[derive(Clone)]
// pub struct HRTSAHandler {
//     inner: Arc<Mutex<Box<dyn Handler>>>,
// }

// impl HRTSAHandler {
//     pub fn new<T: Handler + 'static>(handler: T) -> Self {
//         let key = handler.raw_ptr() as *const huawei_rtsa_ProxyHandler;
//         let this = HRTSAHandler {
//             inner: Arc::new(Mutex::new(Box::new(handler))),
//         };
//         get_register().insert(key, this.clone());
//         this
//     }

//     pub fn raw_ptr(&self) -> *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler {
//         self.inner.lock().expect("lock failed").raw_ptr() as *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler
//     }
// }

// impl Drop for HRTSAHandler {
//     fn drop(&mut self) {
//         if 1 == self.handler.inner.strong_count() {
//             let key  self.inner.lock().expect("lock failed").raw_ptr();
//             get_register().remove(&(self.handler as *const huawei_rtsa_ProxyHandler));
//         }
//     }
// }