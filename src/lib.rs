use std::ffi::c_char;
use hw_rtsa_sdk_sys::{createHRTSAEngine, huawei_rtsa_IHRTSAEventHandler__bindgen_vtable};


struct HRTSAEngine {
    engine: *mut hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEngine,
}

impl HRTSAEngine {
    pub fn new() -> HRTSAEngine {
        unsafe {
            let mut appId: [c_char; 129] = [0; 129];
            appId.copy_from_slice("test".as_bytes().iter().map(|&x| x as c_char).collect::<Vec<_>>().as_slice());
            let mut countryCode: [c_char; 16] = [0; 16];
            let mut logPath: [c_char; 256] = [0; 256];

            let mut param = hw_rtsa_sdk_sys::huawei_rtsa_HRTSACreateParam {
                appId,
                countryCode,
                logPath,
                logFilter: 0,
                logSize: 0,
                relayMode: 0,
                enableEventTracking: false,
                transportMode: 0,
            };

            let mut vtable = hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler__bindgen_vtable {};
            let mut handler = hw_rtsa_sdk_sys::huawei_rtsa_IHRTSAEventHandler {
                vtable_: &mut vtable,
            };
            let engine = createHRTSAEngine(&mut param, &mut handler);
            ((*(*engine).vtable_).huawei_rtsa_IHRTSAEngine_destory)(engine);
            HRTSAEngine { engine }
        }
    }
}