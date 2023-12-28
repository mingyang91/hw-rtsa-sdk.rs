
use hw_rtsa_sdk_sys::{createHandler, huawei_rtsa_ProxyHandler_FFIProtocol};

pub struct HRTSAHandlerInner {
    handler: *mut huawei_rtsa_ProxyHandler,
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

impl huawei_rtsa_ProxyHandler_FFIProtocol for HRTSAHandlerInner {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
