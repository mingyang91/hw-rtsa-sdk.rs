//
// Created by g00522473 on 2021/11/27.
//

#ifndef RTSABUILD_IHRTSAEVENTHANDLER_H
#define RTSABUILD_IHRTSAEVENTHANDLER_H

#include "HRTSAEngineParam.h"
namespace huawei {
namespace rtsa {
class IHRTSAEventHandler {
public:
    // ==================================Common==============================================
    virtual ~IHRTSAEventHandler() = default;
    /* *
     * 本人加入房间成功回调
     *
     * @param userId 用户ID
     * @param roomId 房间号
     * @param elapsed 接口从开始调用到反馈的时间
     */
    virtual void onJoinRoomSuccess(const char *userId, const char *roomId, int elapsed) { }

    virtual void onTransportReady() {}

    virtual void onTransportClosed() {}

    virtual void onTransportNetworkStatUpdate(HRTSATransportNetworkQuality &quality) {}

    /* *
     * 其他人员加入房间
     *
     * @param userId 用户ID
     * @param elapsed 接口从开始调用到反馈的时间
     */
    virtual void onRemoteUserOnline(const char *userId) {}

     /**
     * 用户离线
     *
     * @param userId 用户ID
     * @param reason 错误原因(0是已经离开房间,1是临时断开正在重连中)
     */
    virtual void onRemoteUserOffline(const char *userId, int reason) { }

    /* *
     * 错误码提示
     *
     * @param code 错误标识符
     */
    virtual void onError(int code, const char *msg) { }

    /* *
     * 发生警告回调。
     * 该回调方法表示 SDK 运行时出现了（网络或媒体相关的）警告。通常情况下，SDK 上报的警告信息 App 可以忽略。
     *
     * @param code 警告标识符
     */
    virtual void onWarning(int code, const char *msg) { }

    /* *
     * API 方法已执行回调
     *
     * @param error 错误码。如果方法调用失败，会返回错误码 Error code；如果返回 0，则表示方法调用成功
     * @param api SDK 所调用的 API
     * @param result SDK 调用 API 的调用结果（保留）
     */
    virtual void onApiCalled(int error, const char *api, const char *msg) { }

    /* *
     * 退出房间
     */
    virtual void onLeaveRoom(HRTSALeaveReason reason, const HRTSALeaveStatsInfo *statsInfo) { }

    /* *
     * 该回调在网络连接状态发生改变的时候触发，并告知用户当前的网络连接状态，和引起网络状态改变的原因。
     *
     * @param state 当前状态 NETWORK_STATE_CONNECTED = 0  NETWORK_STATE_RECONNECTED = 1,NETWORK_STATE_FAILED =
     * 2,NETWORK_STATE_RECONNECTING = 3
     */

    /* *
     * token过期回调通知
     */
    virtual void onAuthorizationExpired() { }

    /* *
     * @param reason 原因
     */
    virtual void onConnectionChangedNotify(int state, int reason) { }

    /* *
     * @param 连接状态异常时是否需要重新加入房间
     */
    virtual void onConnectionChangedNeedRejoin(bool& needRejoin) { }

    /* *
     * 媒体层 Socket 句柄回调
     * @param socketFd
     */
    virtual void onMediaSocketFd(int socketFd) { }

    /* *
     * 开始旁推推流结果回调
     * @param code 错误码
     * @param taskId 任务ID
     */
    virtual void onStartPublishStream(int code, const char * taskId) {}

    /* *
     * 更新旁推推流结果回调
     * @param code 错误码
     * @param taskId 任务ID
     */
    virtual void onUpdateTransCoding(int code, const char * taskId) {}

    /* *
     * 停止旁推推流结果回调
     * @param code 错误码
     * @param taskId 任务ID
     */
    virtual void onStopPublishStream(int code, const char * taskId) {}

    /* *
     * RTMP推流状态回调
     * @param code 错误码
     * @param taskId 任务ID
     * @param urlStatus 推流的url状态
     */
    virtual void onStreamPublishStateChangedNotify(int code, const char * taskId, const HRTSAUrlStatusList *urlStatusList) {}

    // ==================================Video==============================================

    /* *
     * 远程视频状态改变
     *
     * @param userId 用户ID
     * @param streamType 流类型
     * @param state 是 流的开关状态
     * @param reason 原因
     */
    virtual void onRemoteVideoStateChangedNotify(const char *userId, const HRTSAStreamType streamType,
        HRTSARemoteVideoStreamState state, HRTSARemoteVideoStreamStateReason reason) { }

    /**
     * 远端通用通道状态改变
     *
     * @param userId 用户ID
     * @param streamType 流类型
     * @param state 是 流的开关状态
     * @param reason 原因
     */
    virtual void onRemoteCommonStateChangedNotify(const char *userId, const HRTSAStreamType streamType,
        HRTSARemoteVideoStreamState state, HRTSARemoteVideoStreamStateReason reason) {}

    /**
     * 视频包回调
     */
    virtual void onRemoteVideoPacketData(const char *userId, const HRTSAStreamType streamType, const unsigned char *packet,
        unsigned int length) { }

    /* *
     * 视频帧回调
     * HRTSAFrameDataOption 收发用同一个结构体
     */
    virtual void onRemoteVideoFrameData(const char *userId, const HRTSAStreamType streamType, const unsigned char *frame,
        unsigned int length, HRTSAFrameVideoOption option) { }

    /**
     * 通用数据包回调
     */
    virtual void onRemoteCommonData(const struct HRTSATransportMsgHdr *msg) {}

    /**
     * 请求关键帧, 收到此回调时建议编码器立即同步编码一个关键帧
     */
    virtual void onRequestKeyFrame(const char *userId, const HRTSAStreamType streamType) { }

    /* *
     * 码率调节回调,收到此回调时app需要根据该参数的值及时调整视频编码器的输出码率
     * 以避免码率超发带来的网络拥塞和视频卡顿。
     * streamType 流类型
     * targetEncBitrate 建议的编码码率 单位 kbps
     */
    virtual void onLocalVideoBitrateUpdate(const HRTSAStreamType streamType, unsigned int targetEncBitrate) { }

    /* *
     * 统计信息 -- 视频发送
     */
    virtual void onLocalVideoStatsNotify(const UpNetworkVideoStatics *connectionStatics,
        const VideoSendStaticsInfo *videoSendStatics) { }

    /* *
     * 统计信息 -- 视频接收
     */
    virtual void onRemoteVideoStatsNotify(const DownNetworkVideoStatics *connectionStatics,
        const VideoRecvStaticsInfo *videoRecvStatics) { }

    /* 上行预估带宽更新,bandwidth是上行的预估总带宽,app收到此回调时需要决策总带宽如何分配给上行的各路流
     * 上行几路流就分配几路流，如果某一路流不分配带宽则码率分配为0。
     * 例如上行有两路流,当前预估带宽为4000, app决定大流分配3900,小流分配100，则将分配结果写入result指针
     * {
     * result->streamInfo[0].streamType = HRTSA_STREAM_TYPE_MAIN; 大流
     * result->streamInfo[0].bitrate = 3900;
     * result->streamInfo[1].streamType = HRTSA_STREAM_TYPE_SLIDES; 小流
     * result->streamInfo[1].bitrate = 100;
     * }
     * localUid 本端用户id
     * result 带宽分配的结果, 应用层分配完各路流之后把结果写入到指针指向的内存
     *
     *  */
    virtual void onLocalVideoBandwidthDistribution(unsigned long bandwidth, HRTSABandwidthDistribution *result) { }

    /**
     * 统计信息 -- 通用数据发送的网络信息
     */
    virtual void onTransportNetworkQuality(const HRTSATransportNetworkQuality *networkQuality) {}

    // ==================================Audio==============================================

    /* *
     * 远程音频状态改变
     *
     * @param uid 用户ID
     * @param streamType 流类型
     * @param muted 是否静音（true静音，false取消静音）
     * @param elapsed 接口从开始调用到反馈的时间
     */

    // HRTSARemoteVideoStreamState结构体中 只有一个mute字段
    virtual void onRemoteAudioStateChangedNotify(const char *userId, HRTSARemoteAudioStreamState state,
        HRTSARemoteAudioStreamStateReason reason) { }

    /* *
     * 音频包回调
     */
    virtual void onRemoteAudioPacketData(const char *userId, const unsigned char *packet, unsigned int length) { }

    /* *
     * 音频帧回调
     */
    virtual void onRemoteAudioFrameData(const char *userId, const unsigned char *frame, unsigned int length,
        HRTSAFrameAudioOption option) { }

    /* *
     * 统计信息 -- 音频发送
     */
    virtual void onLocalAudioStatsNotify(const AudioSendStaticsInfo *audioSendStatics) { }

    /* *
     * 统计信息 -- 音频接收
     */
    virtual void onRemoteAudioStatsNotify(const char *userId, const AudioRecvStaticsInfo *audioRecvStatics) { }

    /* *
     * 最大音量用户
     */
    virtual void onUserVolumeStatsNotify(int totalVolume, int totalUserCount,
        const HRTSAAudioMaxSpeakerInfo *audioMaxSpeakers) { }

    // ==================================Cmd==============================================

    /* *
     * 媒体控制包回调
     */
    virtual void onRemoteCommandPacket(const char *userId, const unsigned char *packet, unsigned int length) { }

    virtual int onAudioEncode(huawei::rtsa::HRTSAAudioEncStru* pstEncPrm) { return 0; }

    virtual int onAudioDecode(huawei::rtsa::HRTSAAudioDecStru* pstDecPrm) { return 0; }
    
    /**
      订阅对端流成功回调
    */
    virtual void onSubStreamSuccess(const char *userId) {}
};
} // namespace rtsa
} // namespace huawei

#endif // RTSABUILD_IHRTSAEVENTHANDLER_H
