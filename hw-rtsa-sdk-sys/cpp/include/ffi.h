#include <stdlib.h>
#include "IHRTSAEngine.h"
#include "IHRTSAEventHandler.h"

// Define a macro to create a wrapper function for a target method
#define PROXY_CALL0(ret, method) \
    ret method() { \
        if (_##method != nullptr) { return _##method(this); } \
        else { return IHRTSAEventHandler::method(); } \
    }
#define PROXY_CALL1(ret, method, type1, name1) \
    ret method(type1 name1) { \
        if (_##method != nullptr) { _##method(this, name1); } \
        else { IHRTSAEventHandler::method(name1); } \
    }
#define PROXY_CALL2(ret, method, type1, name1, type2, name2) \
    ret method(type1 name1, type2 name2) { \
        if (_##method != nullptr) { _##method(this, name1, name2); } \
        else { IHRTSAEventHandler::method(name1, name2); } \
    }
#define PROXY_CALL3(ret, method, type1, name1, type2, name2, type3, name3) \
    ret method(type1 name1, type2 name2, type3 name3) { \
        if (_##method != nullptr) { _##method(this, name1, name2, name3); } \
        else { IHRTSAEventHandler::method(name1, name2, name3); } \
    }
#define PROXY_CALL4(ret, method, type1, name1, type2, name2, type3, name3, type4, name4) \
    ret method(type1 name1, type2 name2, type3 name3, type4 name4) { \
        if (_##method != nullptr) { _##method(this, name1, name2, name3, name4); } \
        else { IHRTSAEventHandler::method(name1, name2, name3, name4); } \
    }
#define PROXY_CALL5(ret, method, type1, name1, type2, name2, type3, name3, type4, name4, type5, name5) \
    ret method(type1 name1, type2 name2, type3 name3, type4 name4, type5 name5) { \
        if (_##method != nullptr) { _##method(this, name1, name2, name3, name4, name5); } \
        else { IHRTSAEventHandler::method(name1, name2, name3, name4, name5); } \
    }
#define PROXY_CALL6(ret, method, type1, name1, type2, name2, type3, name3, type4, name4, type5, name5, type6, name6) \
    ret method(type1 name1, type2 name2, type3 name3, type4 name4, type5 name5, type6 name6) { \
        if (_##method != nullptr) { _##method(this, name1, name2, name3, name4, name5, name6); } \
        else { IHRTSAEventHandler::method(name1, name2, name3, name4, name5, name6); } \
    }

// Utility macro to expand the arguments
#define EXPAND(x) x

// Utility macro to get the correct PROXY_CALL macro based on argument count
#define GET_MACRO(_ret, _method, \
                  _pt1, _pn1, \
                  _pt2, _pn2, \
                  _pt3, _pn3, \
                  _pt4, _pn4, \
                  _pt5, _pn5, \
                  _pt6, _pn6, \
                  NAME, ...) NAME

// Main PROXY_CALL macro to forward to the correct PROXY_CALLx macro
#define PROXY_CALL(...) EXPAND(GET_MACRO(__VA_ARGS__, \
                                         PROXY_CALL6, _PHANTOM, \
                                         PROXY_CALL5, _PHANTOM, \
                                         PROXY_CALL4, _PHANTOM, \
                                         PROXY_CALL3, _PHANTOM, \
                                         PROXY_CALL2, _PHANTOM, \
                                         PROXY_CALL1, _PHANTOM, \
                                         PROXY_CALL0, _PHANTOM)(__VA_ARGS__))



namespace huawei {
namespace rtsa {
class ProxyHandler;

extern "C" {
typedef void (*onJoinRoomSuccessCallback)(const ProxyHandler *self, const char *userId, const char *roomId,
    int elapsed);
typedef void (*onRemoteUserOnlineCallback)(const ProxyHandler *self, const char *userId);
typedef void (*onRemoteUserOfflineCallback)(const ProxyHandler *self, const char *userId, int reason);
typedef void (*onErrorCallback)(const ProxyHandler *self, int code, const char *msg);
typedef void (*onWarningCallback)(const ProxyHandler *self, int code, const char *msg);
typedef void (*onApiCalledCallback)(const ProxyHandler *self, int error, const char *api, const char *msg);
typedef void (*onLeaveRoomCallback)(const ProxyHandler *self, HRTSALeaveReason reason,
    const HRTSALeaveStatsInfo *statsInfo);
typedef void (*onMediaSocketFdCallback)(const ProxyHandler *self, int socketFd);
typedef void (*onTransportReadyCallback)(const ProxyHandler *self);
typedef void (*onTransportClosedCallback)(const ProxyHandler *self);
typedef void (*onTransportNetworkStatUpdateCallback)(const ProxyHandler *self,
    HRTSATransportNetworkQuality &quality);
typedef void (*onAuthorizationExpiredCallback)(const ProxyHandler *self);
typedef void (*onConnectionChangedNotifyCallback)(const ProxyHandler *self, int state, int reason);
typedef void (*onConnectionChangedNeedRejoinCallback)(const ProxyHandler *self, bool& needRejoin);
typedef void (*onRemoteVideoStateChangedNotifyCallback)(const ProxyHandler *self, const char *userId,
    const HRTSAStreamType streamType, HRTSARemoteVideoStreamState state, HRTSARemoteVideoStreamStateReason reason);
typedef void (*onRemoteCommonStateChangedNotifyCallback)(const ProxyHandler *self, const char *userId,
    const HRTSAStreamType streamType, HRTSARemoteVideoStreamState state, HRTSARemoteVideoStreamStateReason reason);
typedef void (*onRemoteVideoPacketDataCallback)(const ProxyHandler *self, const char *userId,
    const HRTSAStreamType streamType, const unsigned char *packet, unsigned int length);
typedef void (*onRemoteVideoFrameDataCallback)(const ProxyHandler *self, const char *userId,
    const HRTSAStreamType streamType, const unsigned char *frame, unsigned int length, HRTSAFrameVideoOption option);
typedef void (*onRemoteCommonDataCallback)(const ProxyHandler *self, const struct HRTSATransportMsgHdr *msg);
typedef void (*onRequestKeyFrameCallback)(const ProxyHandler *self, const char *userId,
    const HRTSAStreamType streamType);
typedef void (*onLocalVideoBitrateUpdateCallback)(const ProxyHandler *self, const HRTSAStreamType streamType,
    unsigned int targetEncBitrate);
typedef void (*onLocalVideoStatsNotifyCallback)(const ProxyHandler *self,
    const UpNetworkVideoStatics *connectionStatics, const VideoSendStaticsInfo *videoSendStatics);
typedef void (*onRemoteVideoStatsNotifyCallback)(const ProxyHandler *self,
    const DownNetworkVideoStatics *connectionStatics, const VideoRecvStaticsInfo *videoRecvStatics);
typedef void (*onLocalVideoBandwidthDistributionCallback)(const ProxyHandler *self, unsigned long bandwidth,
    HRTSABandwidthDistribution *result);
typedef void (*onRemoteAudioStateChangedNotifyCallback)(const ProxyHandler *self, const char *userId,
    HRTSARemoteAudioStreamState state, HRTSARemoteAudioStreamStateReason reason);
typedef void (*onRemoteAudioPacketDataCallback)(const ProxyHandler *self, const char *userId,
    const unsigned char *packet, unsigned int length);
typedef void (*onRemoteAudioFrameDataCallback)(const ProxyHandler *self, const char *userId,
    const unsigned char *frame, unsigned int length, HRTSAFrameAudioOption option);
typedef void (*onLocalAudioStatsNotifyCallback)(const ProxyHandler *self,
    const AudioSendStaticsInfo *audioSendStatics);
typedef void (*onRemoteAudioStatsNotifyCallback)(const ProxyHandler *self, const char *userId,
    const AudioRecvStaticsInfo *audioRecvStatics);
typedef void (*onUserVolumeStatsNotifyCallback)(const ProxyHandler *self, int totalVolume, int totalUserCount,
    const HRTSAAudioMaxSpeakerInfo *audioMaxSpeakers);
typedef void (*onRemoteCommandPacketCallback)(const ProxyHandler *self, const char *userId,
    const unsigned char *packet, unsigned int length);
typedef int (*onAudioEncodeCallback)(const ProxyHandler *self, huawei::rtsa::HRTSAAudioEncStru* pstEncPrm);
typedef int (*onAudioDecodeCallback)(const ProxyHandler *self, huawei::rtsa::HRTSAAudioDecStru* pstDecPrm);
typedef void (*onSubStreamSuccessCallback)(const ProxyHandler *self, const char *userId);
}
class ProxyHandler : public IHRTSAEventHandler {
public:
    onJoinRoomSuccessCallback _onJoinRoomSuccess;
    onTransportReadyCallback _onTransportReady;
    onTransportClosedCallback _onTransportClosed;
    onTransportNetworkStatUpdateCallback _onTransportNetworkStatUpdate;
    onRemoteUserOnlineCallback _onRemoteUserOnline;
    onRemoteUserOfflineCallback _onRemoteUserOffline;
    onErrorCallback _onError;
    onWarningCallback _onWarning;
    onApiCalledCallback _onApiCalled;
    onLeaveRoomCallback _onLeaveRoom;
    onMediaSocketFdCallback _onMediaSocketFd;
    onAuthorizationExpiredCallback _onAuthorizationExpired;
    onConnectionChangedNotifyCallback _onConnectionChangedNotify;
    onConnectionChangedNeedRejoinCallback _onConnectionChangedNeedRejoin;
    onRemoteVideoStateChangedNotifyCallback _onRemoteVideoStateChangedNotify;
    onRemoteCommonStateChangedNotifyCallback _onRemoteCommonStateChangedNotify;
    onRemoteVideoPacketDataCallback _onRemoteVideoPacketData;
    onRemoteVideoFrameDataCallback _onRemoteVideoFrameData;
    onRemoteCommonDataCallback _onRemoteCommonData;
    onRequestKeyFrameCallback _onRequestKeyFrame;
    onLocalVideoBitrateUpdateCallback _onLocalVideoBitrateUpdate;
    onLocalVideoStatsNotifyCallback _onLocalVideoStatsNotify;
    onRemoteVideoStatsNotifyCallback _onRemoteVideoStatsNotify;
    onLocalVideoBandwidthDistributionCallback _onLocalVideoBandwidthDistribution;
    onRemoteAudioStateChangedNotifyCallback _onRemoteAudioStateChangedNotify;
    onRemoteAudioPacketDataCallback _onRemoteAudioPacketData;
    onRemoteAudioFrameDataCallback _onRemoteAudioFrameData;
    onLocalAudioStatsNotifyCallback _onLocalAudioStatsNotify;
    onRemoteAudioStatsNotifyCallback _onRemoteAudioStatsNotify;
    onUserVolumeStatsNotifyCallback _onUserVolumeStatsNotify;
    onRemoteCommandPacketCallback _onRemoteCommandPacket;
    onAudioEncodeCallback _onAudioEncode;
    onAudioDecodeCallback _onAudioDecode;
    onSubStreamSuccessCallback _onSubStreamSuccess;

    PROXY_CALL(void, onJoinRoomSuccess, const char*, userId, const char*, roomId, int, elapsed);
    PROXY_CALL(void, onTransportReady);
    PROXY_CALL(void, onTransportClosed);
    PROXY_CALL(void, onTransportNetworkStatUpdate, HRTSATransportNetworkQuality&, quality);
    PROXY_CALL(void, onRemoteUserOnline, const char*, userId);
    PROXY_CALL(void, onRemoteUserOffline, const char*, userId, int, reason);
    PROXY_CALL(void, onError, int, code, const char*, msg);
    PROXY_CALL(void, onWarning, int, code, const char*, msg);
    PROXY_CALL(void, onApiCalled, int, error, const char*, api, const char*, msg);
    PROXY_CALL(void, onLeaveRoom, HRTSALeaveReason, reason, const HRTSALeaveStatsInfo*, statsInfo);
    PROXY_CALL(void, onMediaSocketFd, int, socketFd);
    PROXY_CALL(void, onAuthorizationExpired);
    PROXY_CALL(void, onConnectionChangedNotify, int, state, int, reason);
    PROXY_CALL(void, onConnectionChangedNeedRejoin, bool&, needRejoin);
    PROXY_CALL(void, onRemoteVideoStateChangedNotify, const char*, userId, const HRTSAStreamType, streamType,
        HRTSARemoteVideoStreamState, state, HRTSARemoteVideoStreamStateReason, reason);
    PROXY_CALL(void, onRemoteCommonStateChangedNotify, const char*, userId, const HRTSAStreamType, streamType,
        HRTSARemoteVideoStreamState, state, HRTSARemoteVideoStreamStateReason, reason);
    PROXY_CALL(void, onRemoteVideoPacketData, const char*, userId, const HRTSAStreamType, streamType,
        const unsigned char*, packet, unsigned int, length);
    PROXY_CALL(void, onRemoteVideoFrameData, const char*, userId, const HRTSAStreamType, streamType,
        const unsigned char*, frame, unsigned int, length, HRTSAFrameVideoOption, option);
    PROXY_CALL(void, onRemoteCommonData, const struct HRTSATransportMsgHdr*, msg);
    PROXY_CALL(void, onRequestKeyFrame, const char*, userId, const HRTSAStreamType, streamType);
    PROXY_CALL(void, onLocalVideoBitrateUpdate, const HRTSAStreamType, streamType, unsigned int, targetEncBitrate);
    PROXY_CALL(void, onLocalVideoStatsNotify, const UpNetworkVideoStatics*, connectionStatics,
        const VideoSendStaticsInfo*, videoSendStatics);
    PROXY_CALL(void, onRemoteVideoStatsNotify, const DownNetworkVideoStatics*, connectionStatics,
        const VideoRecvStaticsInfo*, videoRecvStatics);
    PROXY_CALL(void, onLocalVideoBandwidthDistribution, unsigned long, bandwidth,
        HRTSABandwidthDistribution*, result);
    PROXY_CALL(void, onRemoteAudioStateChangedNotify, const char*, userId, HRTSARemoteAudioStreamState, state,
        HRTSARemoteAudioStreamStateReason, reason);
    PROXY_CALL(void, onRemoteAudioPacketData, const char*, userId, const unsigned char*, packet, unsigned int, length);
    PROXY_CALL(void, onRemoteAudioFrameData, const char*, userId, const unsigned char*, frame, unsigned int, length,
        HRTSAFrameAudioOption, option);
    PROXY_CALL(void, onLocalAudioStatsNotify, const AudioSendStaticsInfo*, audioSendStatics);
    PROXY_CALL(void, onRemoteAudioStatsNotify, const char*, userId, const AudioRecvStaticsInfo*, audioRecvStatics);
    PROXY_CALL(void, onUserVolumeStatsNotify, int, totalVolume, int, totalUserCount,
        const HRTSAAudioMaxSpeakerInfo*, audioMaxSpeakers);
    PROXY_CALL(void, onRemoteCommandPacket, const char*, userId, const unsigned char*, packet, unsigned int, length);
    PROXY_CALL(int, onAudioEncode, huawei::rtsa::HRTSAAudioEncStru*, pstEncPrm);
    PROXY_CALL(int, onAudioDecode, huawei::rtsa::HRTSAAudioDecStru*, pstDecPrm);
    PROXY_CALL(void, onSubStreamSuccess, const char*, userId);
};
} // namespace rtsa
} // namespace huawei


extern "C" huawei::rtsa::ProxyHandler* createHandler();

extern "C" int engine_switchUserAgent(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTCUserAgentType userAgent);
extern "C" int engine_joinRoom(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAJoinParam &joinParam);
extern "C" int engine_setLocalIp(huawei::rtsa::IHRTSAEngine &engine, const char *ip);
extern "C" int engine_setLoggingCallback(huawei::rtsa::IHRTSAEngine &engine, huawei::rtsa::IHRTSATransportLogCallback logging_callback);
extern "C" int engine_setLogLevel(huawei::rtsa::IHRTSAEngine &engine, huawei::rtsa::HRTSATransportLogLevel level);
extern "C" int engine_setTransportType(huawei::rtsa::IHRTSAEngine &engine, huawei::rtsa::HRTSATransportType transportType);
extern "C" int engine_setInitConfig(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAInitConfig &config);
extern "C" int engine_enableFrameForwarding(huawei::rtsa::IHRTSAEngine &engine, bool enable);
extern "C" int engine_enableCommandMsg(huawei::rtsa::IHRTSAEngine &engine, bool enable);
extern "C" int engine_setEventTrackingTag(huawei::rtsa::IHRTSAEngine &engine, char *eventTrackingTag);
extern "C" int engine_setAudioPolicy(huawei::rtsa::IHRTSAEngine &engine, huawei::rtsa::HRTSAAudioPolicy audioPolicy);
extern "C" int engine_setEncryption(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAEncryptionConfig &encryptionConfig);
extern "C" int engine_setNetworkInterface(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSANicParam &param);
extern "C" int engine_setNetworkBandwidth(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSANetworkBandwidth &bandwidthParam);
extern "C" int engine_leaveRoom(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_shutdownTransporter(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_setTransportReceiveBlocked(huawei::rtsa::IHRTSAEngine &engine, bool block);
extern "C" int engine_renewAuthorization(huawei::rtsa::IHRTSAEngine &engine, const char *token, long long ctime);
extern "C" int engine_destory(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_startLocalVideoStream(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_stopLocalVideoStream(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_startRemoteVideoStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_stopRemoteVideoStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_startLocalCommonStream(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_stopLocalCommonStream(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_startRemoteCommonStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_stopRemoteCommonStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_sendVideoPacketData(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType, const unsigned char *packetData, unsigned int length, const huawei::rtsa::HRTSAPacketDataOption &option);
extern "C" int engine_sendVideoFrameData(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType, const unsigned char *frame, unsigned int length, const huawei::rtsa::HRTSAFrameVideoOption &option);
extern "C" int engine_sendCommonData(huawei::rtsa::IHRTSAEngine &engine, const struct huawei::rtsa::HRTSATransportMsgHdr *msg);
extern "C" int engine_preAnalysisVideoFrameData(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAStreamType streamType, const unsigned char *frame, unsigned int length, const huawei::rtsa::HRTSAFrameVideoOption &option, huawei::rtsa::HRTSAFrameVideoAnalysis *info);
extern "C" int engine_requestKeyFrame(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const huawei::rtsa::HRTSAStreamType streamType);
extern "C" int engine_startLocalAudioStream(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_stopLocalAudioStream(huawei::rtsa::IHRTSAEngine &engine);
extern "C" int engine_startRemoteAudioStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId);
extern "C" int engine_stopRemoteAudioStream(huawei::rtsa::IHRTSAEngine &engine, const char *userId);
extern "C" int engine_sendAudioPacketData(huawei::rtsa::IHRTSAEngine &engine, const unsigned char *packetData, unsigned int length, const huawei::rtsa::HRTSAPacketDataOption &option);
extern "C" int engine_sendAudioFrameData(huawei::rtsa::IHRTSAEngine &engine, const unsigned char *frame, unsigned int length, const huawei::rtsa::HRTSAFrameAudioOption &option);
extern "C" int engine_enableUserVolumeNotify(huawei::rtsa::IHRTSAEngine &engine, int interval);
extern "C" int engine_sendCommandData(huawei::rtsa::IHRTSAEngine &engine, const char *userId, const unsigned char *data, unsigned int length);
extern "C" int engine_setAccessResourceType(huawei::rtsa::IHRTSAEngine &engine, char *resourceTags[], int num);
extern "C" int engine_setIntranetEnv(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAIntranetParam &param);
extern "C" int engine_setLocalProxyServer(huawei::rtsa::IHRTSAEngine &engine, const huawei::rtsa::HRTSAProxyParam &param);
extern "C" int engine_startPublishStream(huawei::rtsa::IHRTSAEngine &engine, const char* taskId, const huawei::rtsa::HRTSARtmpUrlList &rtmpUrls, const huawei::rtsa::HRTSATranscodeConfig &transcodeConfig);
extern "C" int engine_updateTransCoding(huawei::rtsa::IHRTSAEngine &engine, const char* taskId, const huawei::rtsa::HRTSATranscodeConfig &transcodeConfig);
extern "C" int engine_stopPublishStream(huawei::rtsa::IHRTSAEngine &engine, const char* taskId);
