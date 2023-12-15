/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: hw rtsa sdk
 */

#ifndef RTSACONTROL_IRTSAENGINE_H
#define RTSACONTROL_IRTSAENGINE_H

#include "HRTSAEngineParam.h"
#include "IHRTSAEventHandler.h"

#ifdef CONTROL_WINDOWS
#define RTSA_ENGINEAPI __declspec(dllexport)
#else
#define RTSA_ENGINEAPI __attribute__((__visibility__("default")))
#endif

namespace huawei {
namespace rtsa {
/**
 * The IRtsaEngine class
 * you can create an 'IRtsaEngine' instance by calling rtsa::IRtsaEngine create(CreateConfig* config, )
 */
class IHRTSAEngine {
public:
    virtual ~IHRTSAEngine() = default;
    // ==================================Common==============================================

    /* *
    * @param userAgent  userAgent类型，对于RTSA场景，枚举是20,只用于linux平台
    * @return 0(success),other(failed)
    */
    virtual int switchUserAgent(const HRTCUserAgentType userAgent) = 0;
    
    /* *
     * 1.用户加入房间，加入房间后，用户可以向房间内发布Audio stream，Video main stream，Video slides stream
     * desktop stream or a media cmd stream。
     * 2.用户也可以订阅已经在房间中发布的流
     * @param appId 应用程序的APPId
     * @param userId 用户ID
     * @param roomId 房间ID
     * @param token jwt鉴权信息
     * @return 0(success),other(failed)
     */
    virtual int joinRoom(const HRTSAJoinParam &joinParam) = 0;

    virtual int SetLocalIp(const char *ip) = 0;

    virtual int setLoggingCallback(IHRTSATransportLogCallback logging_callback) = 0;

    virtual int setLogLevel(HRTSATransportLogLevel level) = 0;

    /**
     * 设置传输类型
     * @param transportType
     * @return 0(success),other(failed)
     */
    virtual int setTransportType(HRTSATransportType transportType) = 0;

    /**
     * 网络配置信息
     *
     * @param config
     * @return 0(success),other(failed)
     */
    virtual int setInitConfig(const HRTSAInitConfig &config) = 0;

    /* *
     * 开启、关闭帧透明传输（帧快速转发）能力
     * 默认 关闭
     * joinroom前调用
     *
     * @param enable true:开启，false:关闭
     * @return 0(success),other(failed)
     */
    virtual int enableFrameForwarding(bool enable) = 0;

    /* *
     * 开启、关闭cmd能力
     * 默认 关闭
     * joinroom前调用
     *
     * @param enable true:开启，false:关闭
     * @return 0(success),other(failed)
     */
    virtual int enableCommandMsg(bool enable) = 0;

    /* *
     * 设置 ha 打点上报实例的 tag
     * joinroom前调用
     *
     * @param enable true:开启，false:关闭
     * @return 0(success),other(failed)
     */
    virtual int setEventTrackingTag(char *eventTrackingTag) = 0;

    /* *
     * 设置 audio 模式
     * 0是手动订阅(默认)，1是自动订阅(TopN)
     * joinroom前调用
     *
     * @param enable true:开启，false:关闭
     * @return 0(success),other(failed)
     */
    virtual int setAudioPolicy(HRTSAAudioPolicy audioPolicy) = 0;

    /* *
     * 设置加密配置信息
     * @param encryptionConfig
     * @return 0(success),other(failed)
     */
    virtual int setEncryption(const HRTSAEncryptionConfig &encryptionConfig) = 0;

    /* *
     *
     * @param param 绑定网卡参数
     * @return 0(success),other(failed)
     */
    virtual int setNetworkInterface(const HRTSANicParam &param) = 0;
    
    /* *
     * 设置网络带宽参数
     * @param bandwidthParam 网络带宽参数
     * @return
     */
    virtual int setNetworkBandwidth(const HRTSANetworkBandwidth &bandwidthParam) = 0;

    /* *
     * 离开房间
     *
     */
    virtual int leaveRoom() = 0;

    virtual int shutdownTransporter() = 0;

    virtual int setTransportReceiveBlocked(bool block) = 0;
    /**
     * 主动刷新认证
     *
     * @param token 签名
     * @param ctime 时间戳
     */
    virtual int renewAuthorization(const char *token, long long ctime) = 0;

    /**
     * 销毁实例
     *
     * @return 0(success),other(failed)
     */
    virtual int destory() = 0;


    // ==================================video==============================================


    /* *
     * 发布一个视频流，可以是MAIN，SLIDES
     *
     * @param streamType 视频流类型
     * @return 0(success),other(failed)
     */
    virtual int startLocalVideoStream(const HRTSAStreamType streamType) = 0;

    /* *
     * 停止发布一个视频流
     *
     * @param streamType 视频流类型
     * @return 0(success),other(failed)
     */
    virtual int stopLocalVideoStream(const HRTSAStreamType streamType) = 0;

    /* *
     * 订阅远端视频流
     *
     * @param userId 远端视频用户id
     * @param streamType 视频流类型
     * @return 0(success),other(failed)
     */
    virtual int startRemoteVideoStream(const char *userId, const HRTSAStreamType streamType) = 0;

    /* *
     * 取消订阅远端视频流
     *
     * @param userId 远端视频用户id
     * @param streamType 视频流类型
     * @return 0(success),other(failed)
     */
    virtual int stopRemoteVideoStream(const char *userId, const HRTSAStreamType streamType) = 0;

    /**
     * 发布一个通用传输流，HRTSAStreamType可选：HRTSA_STREAM_TYPE_OCTET_0 ~ HRTSA_STREAM_TYPE_OCTET_7
     *
     * @param streamType 通用传输流类型
     * @return 0(success),other(failed)
     */
    virtual int startLocalCommonStream(const HRTSAStreamType streamType) = 0;

    /**
     * 停止发布一个通用传输流，HRTSAStreamType可选：HRTSA_STREAM_TYPE_OCTET_0 ~ HRTSA_STREAM_TYPE_OCTET_7
     *
     * @param streamType 通用传输流类型
     * @return 0(success),other(failed)
     */
    virtual int stopLocalCommonStream(const HRTSAStreamType streamType) = 0;

    /**
     * 订阅远端通用传输流，HRTSAStreamType可选：HRTSA_STREAM_TYPE_OCTET_0 ~ HRTSA_STREAM_TYPE_OCTET_7
     *
     * @param userId 远端视频用户id
     * @param streamType 通用传输流类型
     * @return 0(success),other(failed)
     */
    virtual int startRemoteCommonStream(const char *userId, const HRTSAStreamType streamType) = 0;

    /**
     * 取消订阅远端视频流，HRTSAStreamType可选：HRTSA_STREAM_TYPE_OCTET_0 ~ HRTSA_STREAM_TYPE_OCTET_7
     *
     * @param userId 远端视频用户id
     * @param streamType 通用传输流类型
     * @return 0(success),other(failed)
     */
    virtual int stopRemoteCommonStream(const char *userId, const HRTSAStreamType streamType) = 0;

    /**
     * 发送视频包数据
     *
     * @param streamType 视频流类型
     * @param packetData 视频包数据buffer地址
     * @param length 视频包数据buffer长度
     * @param option 扩展参数
     * @return 0(success),other(failed)
     */
    virtual int sendVideoPacketData(const HRTSAStreamType streamType, const unsigned char *packetData, unsigned int length,
        const HRTSAPacketDataOption &option) = 0;

    /* *
     * 发送视频帧数据
     *
     * @param streamType 视频流类型
     * @param frame 视频帧数据buffer地址
     * @param length 视频帧数据buffer长度
     * @param option 扩展参数
     * @return 0(success),other(failed)
     * h264编码器建议采用SVC时域分层编码,因为下行网抗采取丢帧的策略需要SVC编码的配合,否则下行网络拥塞时体验比较差。
     * 针对h264编码, rtsa不支持单独发送sps跟pps,编码器如果I帧不携带pps跟sps,业务需自行在I帧前处理拼接。
     * 帧数据有加密的话，需要先用原始帧数据(未加密的帧数据)调用preAnalysisVideoFrameData接口进行预分析
     * 获取HRTSAFrameVideoAnalysis信息并通过HRTSAFrameVideoOption传递下来。
     */
    virtual int sendVideoFrameData(const HRTSAStreamType streamType, const unsigned char *frame, unsigned int length,
        const HRTSAFrameVideoOption &option) = 0;


    /**
     * 发送通用包数据
     *
     * @param streamType 视频流类型
     * @param packetData 视频包数据buffer地址
     * @param length 视频包数据buffer长度
     * @param option 扩展参数
     * @return 0(success),other(failed)
     */
    virtual int sendCommonData(const struct HRTSATransportMsgHdr *msg) = 0;

    /**
     * 与解析视频帧数据，APP加密场景需要
     *
     * @param streamType 视频流类型
     * @param frame 视频帧数据buffer地址
     * @param length 视频帧数据buffer长度
     * @param option 扩展参数
     * @param info 预分析结果的写入地址
     * @return RTC_FRAME_VIDEO_ANALYSIS
     */
    virtual int preAnalysisVideoFrameData(const HRTSAStreamType streamType, const unsigned char *frame,
        unsigned int length, const HRTSAFrameVideoOption &option, HRTSAFrameVideoAnalysis *info) = 0;

    /* *
     * 发送一个关键帧请求,
     * 解码失败或其他需要请求关键帧的时候调用此方法向数据源请求一个关键帧同步
     * @param userId 订阅远端流的用户id
     * @param streamType 远端流的类型,大流 小流 桌面流
     * @return
     */
    virtual int requestKeyFrame(const char *userId, const HRTSAStreamType streamType) = 0;


    // ==================================Audio==============================================


    /* *
     * 发布一个本地音频流
     *
     * @return 0(success),other(failed)
     */
    virtual int startLocalAudioStream() = 0;


    /* *
     * 停止发布一个本地音频流
     *
     * @return 0(success),other(failed)
     */
    virtual int stopLocalAudioStream() = 0;


    /* *
     * 订阅远端用户的一个音频流
     *
     * @param userId 远端用户ID
     * @return 0(success),other(failed)
     */
    virtual int startRemoteAudioStream(const char *userId) = 0;

    /* *
     * 取消订阅远端用户的音频流
     *
     * @param userId 远端用户ID
     * @return 0(success),other(failed)
     */
    virtual int stopRemoteAudioStream(const char *userId) = 0;

    /* *
     * 发送音频包数据，可以是udp包，rtp包 etc.
     *
     * @param packetData packet数据包
     * @param length 数据长度
     * @param option 扩展选项
     * @return 0(success),other(failed)
     */
    virtual int sendAudioPacketData(const unsigned char *packetData, unsigned int length, const HRTSAPacketDataOption &option) = 0;

    /* *
     * 发送音频帧数据
     * @param frame
     * @param length
     * @param option
     * @return
     */
    virtual int sendAudioFrameData(const unsigned char *frame, unsigned int length, const HRTSAFrameAudioOption &option) = 0;


    /* *
     * 设置音量值上报回调函数的回调周期
     * @param interval 音量值上报周期，默认关闭音量回调, 0：关闭音量回调,有效值范围[100，10000]，单位：毫秒。
     * @return
     */
    virtual int enableUserVolumeNotify(int interval) = 0;


    // ==================================Cmd==============================================

    /* *
     * 发送媒体控制数据包
     *
     * @param userId 远端用户ID
     * @param data
     * @param length
     * @return 0(success),other(failed)
     */
    virtual int sendCommandData(const char *userId, const unsigned char *data, unsigned int length) = 0;

    // ==================================Private==============================================
    /* *
     * 资源标签
     * joinroom前调用
     *
     * @param resourceTags:资源标签字符串
     * @param num:资源标签数量
     * @return 0(success),other(failed)
     */
    virtual int setAccessResourceType(char *resourceTags[], int num) = 0;

    /* *
     * 设置内网标志
     * joinroom前调用
     * @param:内网相关参数
     * @return 0(success),other(failed)
     */
    virtual int setIntranetEnv(const HRTSAIntranetParam &param) = 0;

    /* *
     * 设置代理参数
     * joinroom前调用
     * @param:代理相关参数
     * @return 0(success),other(failed)
     */
    virtual int setLocalProxyServer(const HRTSAProxyParam &param) = 0;

    /**
     * @brief 开始旁路直播任务
     *
     * @param taskId 任务ID
     * @param rtmpUrls 推流地址
     * @param transcodeConfig 转码配置参数
     * @return 0: 成功, 非0: 失败
     */
    virtual int startPublishStream(const char* taskId, const HRTSARtmpUrlList &rtmpUrls, const HRTSATranscodeConfig &transcodeConfig) = 0;

    /**
     * @brief 更新旁路直播任务
     *
     * @param taskId 任务ID
     * @param transcodeConfig 转码配置参数
     * @return 0: 成功, 非0: 失败
     */
    virtual int updateTransCoding(const char* taskId, const HRTSATranscodeConfig &transcodeConfig) = 0;

    /**
     * @brief 停止旁路直播任务
     *
     * @param taskId 任务ID
     * @return 0: 成功, 非0: 失败
     */
    virtual int stopPublishStream(const char* taskId) = 0;

};
} // namespace rtsa
} // namespace huawei

RTSA_ENGINEAPI huawei::rtsa::IHRTSAEngine *createHRTSAEngine(huawei::rtsa::HRTSACreateParam *createParam,
    huawei::rtsa::IHRTSAEventHandler *rtsaEventHandler);

#endif // RTSACONTROL_IRTSAENGINE_H
