use std::ffi::c_char;
use std::mem::{transmute};
use hw_rtsa_sdk_sys::{huawei_rtsa_HRTSACreateParam};
use crate::utils::ToFixedBytes;

pub enum RelayMode {
    Frame = 0,
    Packet = 1,
}

impl Default for RelayMode {
    fn default() -> Self {
        RelayMode::Frame
    }
}

pub struct HRTSAParam {
    pub app_id: [c_char; 129],
    pub country_code: [c_char; 16],
    pub log_path: [c_char; 256],
    pub log_filter: LogFilter,
    pub log_size: i32,
    pub relay_mode: RelayMode,
    pub enable_event_tracking: bool,
    pub transport_mode: i32,
}

pub enum LogFilter {
    None = -1,
    Trace = 1,
    Verbose = 2,
    Debug = 3,
    Info = 4,
    Warning = 5,
    Error = 6,
}

impl Default for LogFilter {
    fn default() -> Self {
        LogFilter::None
    }
}

impl Into<huawei_rtsa_HRTSACreateParam> for HRTSAParam {
    fn into(self) -> huawei_rtsa_HRTSACreateParam {
        // let app_id = self.app_id.to_fixed_bytes();
        // let mut app_id: [u8; 129] = [0; 129];
        // app_id.copy_from_slice(self.app_id.as_bytes());
        // let mut country_code: [u8; 16] = [0; 16];
        // country_code.copy_from_slice(self.country_code.as_bytes());
        // let mut log_path: [u8; 256] = [0; 256];
        // log_path.copy_from_slice(self.log_path.as_bytes());

        unsafe {
            huawei_rtsa_HRTSACreateParam {
                appId: app_id,
                countryCode: country_code,
                logPath: log_path,
                logFilter: self.log_filter as i32,
                logSize: self.log_size,
                relayMode: self.relay_mode as i32,
                enableEventTracking: self.enable_event_tracking,
                transportMode: self.transport_mode,
            }
        }
    }
}

impl Default for HRTSAParam {
    fn default() -> Self {
        HRTSAParam {
            app_id: Default::default(),
            country_code: Default::default(),
            log_path: Default::default(),
            log_filter: LogFilter::None,
            log_size: 30,
            relay_mode: RelayMode::Frame,
            enable_event_tracking: true,
            transport_mode: 0,
        }
    }
}

impl HRTSAParam {
    fn set_app_id(mut self, app_id: String) -> Option<Self> {
        let app_id = app_id.to_fixed_bytes()?;
        self.app_id.copy_from_slice(&app_id.unwrap());
        Some(self)
    }
}