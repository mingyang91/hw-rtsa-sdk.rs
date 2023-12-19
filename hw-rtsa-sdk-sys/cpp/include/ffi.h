#include <stdlib.h>
#include "IHRTSAEngine.h"
#include "IHRTSAEventHandler.h"


namespace huawei {
namespace rtsa {
class ProxyHandler;

typedef void (*onJoinRoomSuccessCallback)(const ProxyHandler *self, const char *userId, const char *roomId, int elapsed);
typedef void (*onRemoteUserOnlineCallback)(const ProxyHandler *self, const char *userId);
typedef void (*onRemoteUserOfflineCallback)(const ProxyHandler *self, const char *userId, int reason);
typedef void (*onErrorCallback)(const ProxyHandler *self, int code, const char *msg);
typedef void (*onWarningCallback)(const ProxyHandler *self, int code, const char *msg);
typedef void (*onApiCalledCallback)(const ProxyHandler *self, int error, const char *api, const char *msg);
typedef void (*onLeaveRoomCallback)(const ProxyHandler *self, HRTSALeaveReason reason, const HRTSALeaveStatsInfo *statsInfo);
typedef void (*onMediaSocketFdCallback)(const ProxyHandler *self, int socketFd);

class ProxyHandler : public IHRTSAEventHandler {
public:
    void onJoinRoomSuccess(const char *userId, const char *roomId, int elapsed) {
        if (_onJoinRoomSuccess != nullptr) _onJoinRoomSuccess(this, userId, roomId, elapsed);
        else IHRTSAEventHandler::onJoinRoomSuccess(userId, roomId, elapsed);
    }
    onJoinRoomSuccessCallback _onJoinRoomSuccess;

    void onRemoteUserOnline(const char *userId) {
        if (_onRemoteUserOnline != nullptr) _onRemoteUserOnline(this, userId);
        else IHRTSAEventHandler::onRemoteUserOnline(userId);
    }
    onRemoteUserOnlineCallback _onRemoteUserOnline;

    void onRemoteUserOffline(const char *userId, int reason) {
        if (_onRemoteUserOffline != nullptr) _onRemoteUserOffline(this, userId, reason);
        else IHRTSAEventHandler::onRemoteUserOffline(userId, reason);
    }
    onRemoteUserOfflineCallback _onRemoteUserOffline;

    void onError(int code, const char *msg) {
        if (_onError != nullptr) _onError(this, code, msg);
        else IHRTSAEventHandler::onError(code, msg);
    }
    onErrorCallback _onError;

    void onWarning(int code, const char* msg) {
        if (_onWarning != nullptr) _onWarning(this, code, msg);
        else IHRTSAEventHandler::onWarning(code, msg);
    }
    onWarningCallback _onWarning;

    void onApiCalled(int error, const char* api, const char* msg) {
        if (_onApiCalled != nullptr) _onApiCalled(this, error, api, msg);
        else IHRTSAEventHandler::onApiCalled(error, api, msg);
    }
    onApiCalledCallback _onApiCalled;

    void onLeaveRoom(HRTSALeaveReason reason, const HRTSALeaveStatsInfo* statsInfo) {
        if (_onLeaveRoom != nullptr) _onLeaveRoom(this, reason, statsInfo);
        else IHRTSAEventHandler::onLeaveRoom(reason, statsInfo);
    }
    onLeaveRoomCallback _onLeaveRoom;

    void onMediaSocketFd(int socketFd) {
        if (_onMediaSocketFd != nullptr) _onMediaSocketFd(this, socketFd);
        else IHRTSAEventHandler::onMediaSocketFd(socketFd);
    }
    onMediaSocketFdCallback _onMediaSocketFd;
};
} // namespace rtsa
} // namespace huawei


extern "C" huawei::rtsa::ProxyHandler* createHandler();
extern "C" int engine_setInitConfig(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAInitConfig &config);
extern "C" int engine_joinRoom(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAJoinParam &joinRoom);
extern "C" int engine_leaveRoom(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_destory(huawei::rtsa::IHRTSAEngine &engine);
