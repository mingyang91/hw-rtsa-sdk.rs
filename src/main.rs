use std::env::current_dir;

use hw_rtsa_sdk::engine::HRTSAEngine;
use hw_rtsa_sdk::handler::HRTSAHandler;
use hw_rtsa_sdk::param::HRTSAParam;
use thiserror::Error;

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
    let handler = HRTSAHandler::new();
    let engine = HRTSAEngine::new(param, handler)?;
    println!("created");
    let ret = engine.join_room("room_id", "user_id")?;
    engine.leave_room();
    println!("joined");

    Ok(())
}
