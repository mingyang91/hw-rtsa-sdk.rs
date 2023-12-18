use playground::engine::HRTSAEngine;
use playground::handler::HRTSAHandler;
use playground::param::HRTSAParam;

#[tokio::main]
async fn main() {
    let param: HRTSAParam = Default::default();
    let handler = HRTSAHandler::new();
    let engine = HRTSAEngine::new(param, handler);
    engine.join_room("room_id", "user_id");
}
