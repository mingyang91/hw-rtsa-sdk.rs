use std::mem::transmute;
use hw_rtsa_sdk_sys::huawei_rtsa_HRTSACreateParam;
use crate::utils::ToFixedBytes;
use thiserror::Error;

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
    pub app_id: String,
    pub country_code: String,
    pub log_path: String,
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

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("string `{str}` to long to field `{name}[{max_size}]`")]
    StringToLong { name: String, max_size: usize, str: String }
}

impl TryInto<huawei_rtsa_HRTSACreateParam> for HRTSAParam {
    type Error = ConvertError;
    fn try_into(self) -> Result<huawei_rtsa_HRTSACreateParam, Self::Error> {
        let app_id: [u8; 129] = self.app_id.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "app_id".to_string(), max_size: 129, str: self.app_id })?;
        let country_code: [u8; 16] = self.country_code.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "country_code".to_string(), max_size: 16, str: self.country_code })?;
        let log_path: [u8; 256] = self.log_path.clone().to_fixed_bytes()
            .ok_or(ConvertError::StringToLong { name: "log_path".to_string(), max_size: 256, str: self.log_path })?;

        unsafe {
            let param = huawei_rtsa_HRTSACreateParam {
                appId: transmute(app_id),
                countryCode: transmute(country_code),
                logPath: transmute(log_path),
                logFilter: self.log_filter as i32,
                logSize: self.log_size,
                relayMode: self.relay_mode as i32,
                enableEventTracking: self.enable_event_tracking,
                transportMode: self.transport_mode,
            };
            Ok(param)
        }
    }
}

impl Default for HRTSAParam {
    fn default() -> Self {
        HRTSAParam {
            app_id: Default::default(),
            country_code: Default::default(),
            log_path: Default::default(),
            log_filter: LogFilter::Trace,
            log_size: 30,
            relay_mode: RelayMode::Frame,
            enable_event_tracking: true,
            transport_mode: 0,
        }
    }
}
