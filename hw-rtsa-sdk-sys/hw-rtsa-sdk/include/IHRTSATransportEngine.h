/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2022. All rights reserved.
 * Description: hw rtsa sdk
 */

#ifndef RTSACONTROL_TRANSPORT_H
#define RTSACONTROL_TRANSPORT_H

#include "HRTSAEngineParam.h"
#include <cstdint>
#include <string>

#ifdef CONTROL_WINDOWS
#define RTSA_TRANSPORT_API __declspec(dllexport)
#else
#define RTSA_TRANSPORT_API __attribute__((__visibility__("default")))
#endif

#define HRTSA_TRANSPORT_MSG_DONTWAIT 1

namespace huawei {
namespace rtsa {

using HRTSAConnectParam = HRTSAJoinParam;
using HRTSAEndpointInfoList = HRTSAUserInfoList;

class IHRTSATransportEngine {
public:
    virtual ~IHRTSATransportEngine() = default;

    /**
     * 绑定本地网卡
     * @param param 本地网卡名称
     * @return 0(HRTSA_TRANS_SUCCESS), other(HRTSATransportErrorCode)
     */
    virtual int32_t bind(HRTSANicParam &param) = 0;

    /*
     * 设置日志回调
     * @param logging_callback日志回调函数
     */
    virtual void setLoggingCallback(IHRTSATransportLogCallback callback) = 0;

    /*
     * 设置日志级别
     * @param level日志回调级别，不设置默认HRTSA_CLIENT_ULOG_LEVEL_DEBUG
     */
    virtual void setLogLevel(HRTSATransportLogLevel level) = 0;

    /**
     * 1.用户加入房间，创建Quic连接;
     *  2.加入房间后，用户可以通过createStream发布HRTSATransportPriority 0~7 不同优先级的流
     * 3.用户也可以订阅已经在房间中发布的流
     * @param appId 应用程序的APPId
     * @param userId 用户ID
     * @param roomId 房间ID
     * @param token jwt鉴权信息
     * @return 0(success),other(failed)
     */
    virtual int connect(const HRTSAConnectParam &joinParam) = 0;

    /**
     * 设置环境配置信息, debug场景需要
     *
     * @param flavor, gammagreen / gammagreen4等，调试使用
     */
    virtual void setFlavor(std::string flavor) = 0;

    /**
     * 网络配置信息
     *
     * @param config
     * @return 0(success),other(failed)
     */
    virtual int setInitConfig(const HRTSAInitConfig &config) = 0;

    /**
     * 获取connect是否完毕
     * @return bool 是否connect sfu连接完成
     * @return code 连接结果的详细Code, 0(HRTSA_TRANS_SUCCESS)代表加入成功
     */
    virtual bool isConnectingDone(int32_t &code) = 0;

    /**
     * 创建并发布stream
     * @param priority, 流优先级[0 ~ 7]，每个priority stream只能创建一个
     * @return 0(success), other(failed), 重复建立相同的priority，返回HRTSA_TRANS_STREAM_ALREADY_EXIST
     */
    virtual int createStream(const HRTSATransportPriority priority) = 0;

    /**
     * 获取同一个roomId中，自身已经订阅的stream的对端userId和priority
     * @return 已经上线的用户信息
     */
    virtual void getLocalSubscribeInfo(HRTSASubscribeInfoList *points) = 0;

    /**
     * 获取同一个roomId中，订阅了本地指定priority stream的对端userId (待定)
     * @return 已经上线的用户信息
     */
    virtual void getRemoteSubscribeInfo(HRTSASubscribeInfoList *points) = 0;

    /**
     * 关闭读、写操作, 不关闭Quic连接，用于优雅退出连接
     * @return 0(HRTSA_TRANS_SUCCESS), other(HRTSATransportErrorCode)
     */
    virtual int32_t shutdown() = 0;

    /**
     * 获取shutDown的状态，对端是否接收完成
     * @return true(shutdown完成), false(shutdown未完成)
     */
    virtual bool getShutdownStatus() = 0;

    /**
     * 关闭Quic连接，退出房间
     * @return 0(HRTSA_TRANS_SUCCESS), other(HRTSATransportErrorCode)
     */
    virtual int32_t close() = 0;

    /**
     * 发送数据接口, 默认阻塞接口, 通过 flags |= HRTSA_TRANSPORT_MSG_DONTWAIT修改为非阻塞接口
     * @param HRTSATransportMsgHdr *msg, 需要发送的数据
     * @return <0(HRTSATransportErrorCode), >0(sendLen)
     */
    virtual int32_t sendMsg(const struct HRTSATransportMsgHdr *msg, int flags) = 0;

    /**
     * 接收数据接口, 默认阻塞接口, 通过 flags |= HRTSA_TRANSPORT_MSG_DONTWAIT修改为非阻塞接口
     * 优先回调高优先级recv
     * 无userId概念，限制P2P场景使用，否则会导致多userId数据混淆提交
     * @param  buf : 接收缓冲区
     * @param  len : 接收缓冲区长度
     * @param  priority : 接收指定priority的数据
     * @param  flags : 传输flags
     * @param  timeout : 阻塞模式下的超时时间, 单位为ms; 0代表无超时时间，一直阻塞; 超时返回TRANS_ERROR_CODE_EAGAIN
     * @return <0(HRTSATransportErrorCode), =0(unitrans::TRANS_ERROR_CODE_STREAM_FIN正常关闭), >0(readLen)
     */
    virtual int32_t recv(void *buf, int32_t len, HRTSATransportPriority priority, int flags, int timeout) = 0;

    /**
     * 接收数据接口，默认阻塞接口, 通过 flags |= HRTSA_TRANSPORT_MSG_DONTWAIT修改为非阻塞接口
     * @param 传入HRTSATransportMsgHdr *msg, 然后
     * @return的msg内部iov数据内存由底层new，上层使用后需要自行free
     * @param  timeout : 阻塞模式下的超时时间, 单位为ms; 0代表无超时时间，一直阻塞; 超时返回TRANS_ERROR_CODE_EAGAIN
     * @return <0(HRTSATransportErrorCode), =0(unitrans::TRANS_ERROR_CODE_STREAM_FIN正常关闭), >0(readLen)
     */
    virtual int32_t recvMsg(struct HRTSATransportMsgHdr *msg, int flags, int timeout) = 0;

    /**
     * 获取发送侧的传输网络状态
     * @return Unitrans传输统计信息
     */
    virtual void getTransportNetworkQuality(HRTSATransportNetworkQuality *quality) = 0;

    /**
     * 获取在线用户数
     * @return 在线用户个数
     */
    virtual uint32_t getOnlineEndpointNum() = 0;

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
     * 主动刷新认证
     *
     * @param token 签名
     * @param ctime 时间戳
     */
    virtual int renewAuthorization(const char *token, long long ctime) = 0;

};
}  // namespace rtsa
}  // namespace huawei

RTSA_TRANSPORT_API huawei::rtsa::IHRTSATransportEngine *createHRTSATransportEngine(
    huawei::rtsa::HRTSACreateParam *createParam);

#endif  // RTSACONTROL_IRTSAENGINE_H