
use hw_rtsa_sdk_sys::bindings::huawei_rtsa_ProxyHandler_FFIProtocol;

pub struct HRTSAHandlerInner {}

impl HRTSAHandlerInner {
    pub fn new() -> Self {
        HRTSAHandlerInner {}
    }
}

impl huawei_rtsa_ProxyHandler_FFIProtocol for HRTSAHandlerInner {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
