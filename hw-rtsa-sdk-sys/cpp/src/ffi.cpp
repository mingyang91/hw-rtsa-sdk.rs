#include<ffi.h>
#include <iostream>

using namespace huawei::rtsa;
extern "C" ProxyHandler* createHandler() {
    return new ProxyHandler();
};

extern "C" int engine_setInitConfig(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAInitConfig &config) {
    return engine.setInitConfig(config);
};

extern "C" int engine_joinRoom(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAJoinParam &joinRoom) {
    return engine.joinRoom(joinRoom);
};

extern "C" int engine_leaveRoom(huawei::rtsa::IHRTSAEngine &engine) {
    return engine.leaveRoom();
};

extern "C" int engine_destory(huawei::rtsa::IHRTSAEngine &engine) {
    return engine.destory();
};
