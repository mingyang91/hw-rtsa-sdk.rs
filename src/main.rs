use std::env::current_dir;
use std::time::Duration;

use hw_rtsa_sdk::engine::HRTSAEngine;
use hw_rtsa_sdk::handler::{HRTSAHandlerInner};
use hw_rtsa_sdk::param::HRTSAParam;
use hw_rtsa_sdk_sys::bindings::{huawei_rtsa_ProxyHandler_FFIAgent};
use thiserror::Error;
use tokio::time::sleep;
use hw_cloud_schema::存储::OBS::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("engine error")]
    Engine(#[from] hw_rtsa_sdk::engine::CreateError),
    #[error("join error")]
    JoinError(#[from] hw_rtsa_sdk::engine::EngineError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut param: HRTSAParam = Default::default();
    param.app_id = "app_id".to_string();
    param.country_code = "cn".to_string();
    let pwd = current_dir().expect("get current dir error");
    
    param.log_path = pwd.display().to_string();
    let real_handler = HRTSAHandlerInner::new();
    let handler = huawei_rtsa_ProxyHandler_FFIAgent::new(real_handler);
    let engine = HRTSAEngine::new(param, handler)?;
    println!("created");
    let ret = engine.join_room("room_id", "user_id")?;
    println!("join room");
    sleep(Duration::from_secs(10)).await;
    // engine.leave_room();

    Ok(())
}
