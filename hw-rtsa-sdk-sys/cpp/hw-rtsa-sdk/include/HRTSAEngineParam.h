#ifndef RTSABUILD_HRTSAENGINEPARAM_H
#define RTSABUILD_HRTSAENGINEPARAM_H

#include <vector>
#include <cstdarg>
#include <stdint.h>
namespace huawei {
namespace rtsa {
// TODO 需要确定各个结构体中字段的初始值


const int RTSA_MAX_APPID_LEN = 129;
const int RTSA_MAX_USERID_LEN = 65;
const int RTSA_MAX_ROOMID_LEN = 65;
const int RTSA_MAX_SIGNATURE_LEN = 2049;
const int RTSA_MAX_COUNTRY_CODE_LEN = 16;
const int RTSA_MAX_UP_STREAM_NUM = 3;
const int RTSA_MAX_DOWN_STREAM_NUM = 25;
const int RTSA_MAX_SPEAKER_NUM = 16;
const int RTSA_MAX_PATH = 256;
const int RTSA_MAX_NICK_NAME_LEN = 128;
const int RTSA_MAX_INTRANET_IP_LEN = 64;
const int RTSA_MAX_INTRANET_IP_NUM = 2;
const int RTSA_MAX_USERNAME_LEN = 65;
const int RTSA_MAX_PAGE_NUM = 100;
const int RTSA_MAX_TRANSPORT_ENDPOINT_NUM = 32;
const int RTSA_MAX_PROXY_URL_LEN = 257;
const int RTSA_MAX_PROXY_NAME_LEN = 17;
const int RTSA_MAX_PROXY_PWD_LEN = 65;
const int RTSA_MAX_RTMP_URL_LEN = 1024;
const int RTSA_MAX_RTMP_URL_NUM = 5;
const int RTSA_MAX_RTMP_USER_NUM = 50;
const int RTSA_AUDIO_SAMPLE_RATE_16KHZ = 16000;
const int RTSA_AUDIO_SAMPLE_RATE_32KHZ = 32000;
const int RTSA_AUDIO_SAMPLE_RATE_44KHZ = 44100;
const int RTSA_AUDIO_SAMPLE_RATE_48KHZ = 48000;

enum HRTCUserAgentType {
    AGENT_TYPE_UNKNOWN = 0,
    AGENT_TYPE_NATIVE = 1,
    AGENT_TYPE_RECORD = 20
};

enum HRTSAScenarioType {
    /* *
     * 加入房间时的参数：场景
     * 0:小会场（默认）
     * 1:大会场（千人-RTSA不支持）
     * 2:p2p（RTSA不支持）
     * 3: 畅连专用
     */
    HRTSA_SCENARIO_NORMAL = 0,
    HRTSA_SCENARIO_LARGE = 1,
    HRTSA_SCENARIO_P2P = 2,
    HRTSA_SCENARIO_AUTO_CMD = 3
};

// 流类型：大流、小流等
enum HRTSAStreamType {
    HRTSA_STREAM_TYPE_UNKNOWN = 0,
    HRTSA_STREAM_TYPE_MAIN = 1,
    HRTSA_STREAM_TYPE_SLIDES = 2,
    HRTSA_STREAM_TYPE_DESKTOP = 3,
    HRTSA_STREAM_TYPE_LD = 4,  /* 流畅 */
    HRTSA_STREAM_TYPE_SD = 5,  /* 标清 middle3 */
    HRTSA_STREAM_TYPE_HD = 6,  /* 高清 middle2 */
    HRTSA_STREAM_TYPE_FHD = 7, /* 超高清 middle1 */
    HRTSA_STREAM_TYPE_THD = 8,
    HRTSA_STREAM_TYPE_OCTET_0 = 9,       /*cmd*/
    HRTSA_STREAM_TYPE_OCTET_1 = 10,      /*octet-stream1*/
    HRTSA_STREAM_TYPE_OCTET_2 = 11,      /*octet-stream2*/
    HRTSA_STREAM_TYPE_OCTET_3 = 12,      /*octet-stream3*/
    HRTSA_STREAM_TYPE_OCTET_4 = 13,      /*octet-stream4*/
    HRTSA_STREAM_TYPE_OCTET_5 = 14,      /*octet-stream5*/
    HRTSA_STREAM_TYPE_OCTET_6 = 15,      /*octet-stream6*/
    HRTSA_STREAM_TYPE_OCTET_7 = 16,      /*octet-stream7*/
    HRTSA_STREAM_TYPE_BUFF
};

enum HRTSACryptionMode {
    HRTSA_CRYPTO_DEFAULT = 0, //  非端到端加密 0,不开启端到端加密，此时srtp认证(包校验）+加密，当前仅支持一种模式
    HRTSA_CRYPTO_AUTHENTICATION_SDK = 1, // 1，SDK加密 开启端到端加密，srtp只认证(包校验），sdk内部加密, 必现配置key
    HRTSA_CRYPTO_AUTHENTICATION_APP = 2, // 2，APK加密 开启端到端加密，srtp只认证(包校验），应用层加密，需注册回调
    HRTSA_CRYPTO_CUSTOM1 = 3 // 开启端到端加密，此时媒体数据不加密，通道srtp认证+加密。
};

enum HRTSASuiteType {
    HRTSA_ENCRYPTION_128_CTR = 0 // 当前SDK内部加密只支持此种算法
};

enum HRTSACryptionSecFormat {
    HRTSA_HEX_STRING = 0, // 默认支持密钥是HEX格式化
};

// 绑定网卡时传递网卡类型
enum HRTSANicType {
    NIC_TYPE_UNKNOWN = 0,
    NIC_TYPE_MOBILE = 1,
    NIC_TYPE_WLAN = 2
};

enum HRTSALeaveReason {
    HRTSA_LEAVE_REASON_USER_LEAVE_ROOM = 0, // 用户主动离开
    HRTSA_LEAVE_REASON_INTERRUPTED = 1,     // 临时断开正在重连中
    HRTSA_LEAVE_REASON_KICKED_OFF = 2,      // 被踢
    HRTSA_LEAVE_REASON_ROOM_DISMISSED = 3,  // 房间被解散
    HRTSA_LEAVE_REASON_RECONNECT_FAILED = 4, // 重连超时
    HRTSA_LEAVE_REASON_SIGNATURE_EXPIRED = 5,  // 签名过期
    HRTSA_LEAVE_REASON_SERVER_ERROR = 6,       // 服务器异常
    HRTSA_LEAVE_REASON_REGION_NOT_COVERED = 7, // 不在服务区域
};

enum HRTSAVideoFrameType {
    HRTSA_VIDEO_VIDEO_FRAME_KEY = 3,    // 关键帧
    HRTSA_VIDEO_VIDEO_FRAME_DELTA = 4,  // SVC编码的小P帧,非解码参考帧,弱网下可丢弃
    HRTSA_VIDEO_VIDEO_FRAME_ALTREF = 6, // SVC编码的大P帧,解码参考帧,不可丢弃
};

enum HRTSARemoteVideoStreamState {
    HRTSA_REMOTE_VIDEO_STATE_STOPPED = 0,
    HRTSA_REMOTE_VIDEO_STATE_STARTING = 1
};

enum HRTSARemoteVideoStreamStateReason {
    HRTSA_REMOTE_VIDEO_REASON_REMOTE_MUTED = 0,
    HRTSA_REMOTE_VIDEO_REASON_REMOTE_UNMUTED = 1
};

enum HRTSARemoteAudioStreamState {
    HRTSA_REMOTE_AUDIO_STATE_STOPPED = 0,
    HRTSA_REMOTE_AUDIO_STATE_STARTING = 1
};

enum HRTSARemoteAudioStreamStateReason {
    HRTSA_REMOTE_AUDIO_REASON_REMOTE_MUTED = 0,
    HRTSA_REMOTE_AUDIO_REASON_REMOTE_UNMUTED = 1
};

enum HRTSARemoteCommandStreamState {
    HRTSA_REMOTE_CMD_STATE_STOPPED = 0,
    HRTSA_REMOTE_CMD_STATE_STARTING = 1
};

enum HRTSARemoteCommandStreamStateReason {
    HRTSA_REMOTE_CMD_REASON_REMOTE_MUTED = 0,
    HRTSA_REMOTE_CMD_REASON_REMOTE_UNMUTED = 1
};

enum HRTSAAudioPolicy {
    HRTSA_AUDIO_POLICY_DEFAULT = 0,
    HRTSA_AUDIO_POLICY_TOPN = 1,
    HRTSA_AUDIO_POLICY_BUFF
};

// ====================================common=======================================


// Create Engine 的参数
struct HRTSACreateParam {
    /* *
     * appId
     */
    char appId[RTSA_MAX_APPID_LEN];

    /* *
     * 国家码
     */
    char countryCode[RTSA_MAX_COUNTRY_CODE_LEN];

    /* *
     * 日志路径
     * the config of root log path
     * the sdk will add a subdirectory with userid.
     */
    char logPath[RTSA_MAX_PATH];

    /* *
     * the level of log print.
     * -1: do not print
     * 3: debug info warn error
     * 4: info warn error
     * 5: warn error
     * 6: error
     */
    int logFilter = 0;

    /* *
     * the config of log file size.
     */
    int logSize = 30;

    /* *
     * the relay mode. 0:frame, 1:packet data
     * the instance can not change mode after it is created;
     * 0 帧模式 1包模式
     */
    int relayMode = 0;

    /* *
     * 是否开启打点上报能力
     */
    bool enableEventTracking = true;

    /* *
     * 仅P2P场景使用，限制房间内所有人使用同一传输技术栈，不能互通
     * 默认值0表示传输技术栈可以兼容，能够互通
     */
    int transportMode = 0;
};

// 网络带宽参数
struct HRTSANetworkBandwidth {
    int maxTotalBandwith;   // 最大总带宽，有效值范围[3072，51200]，单位：Kbps。
};

// 加入房间的参数
struct HRTSAJoinParam {
    char appId[RTSA_MAX_APPID_LEN];
    char token[RTSA_MAX_SIGNATURE_LEN];
    char userId[RTSA_MAX_USERID_LEN];
    char roomId[RTSA_MAX_ROOMID_LEN];
    long long ctime = 0;
    /* *
     * 会场模式：0 小会场（默认），1 大会场（千人），2 P2P ，3 自动推和订阅cmd流
     */
    HRTSAScenarioType scenario = HRTSA_SCENARIO_NORMAL;
};

struct HRTSAPacketDataOption {
    int reserve;
};

// 绑定网卡参数
struct HRTSANicParam {
    char nicName[RTSA_MAX_NICK_NAME_LEN]; // 网卡名，network interface controller，NIC
    HRTSANicType type = NIC_TYPE_UNKNOWN; // 网卡类型
};

// 内网目的地址相关参数
struct HRTSAIntranetParam {
    char ctlIp[RTSA_MAX_INTRANET_IP_NUM][RTSA_MAX_INTRANET_IP_LEN];
    int ctlIpNum;
    char haIp[RTSA_MAX_INTRANET_IP_NUM][RTSA_MAX_INTRANET_IP_LEN];
    int haIpNum;
    char logServerIp[RTSA_MAX_INTRANET_IP_NUM][RTSA_MAX_INTRANET_IP_LEN];
    int logServerIpNum;
};

// 代理相关参数
struct HRTSAProxyParam {
    char proxyUrl[RTSA_MAX_PROXY_URL_LEN] {0};
    int  proxyPort {0};
    char proxyUserName[RTSA_MAX_PROXY_NAME_LEN] {0};
    char proxyUserPassWord[RTSA_MAX_PROXY_PWD_LEN] {0};
};

// 初始化参数：设置证书、环境等
struct HRTSAInitConfig {
    char grsRootPath[RTSA_MAX_PATH]; // grs 兜底文件路径../../nk-grs
    char caFilePath[RTSA_MAX_PATH];  // websocket证书路径
};

enum HRTSATransportType {
    HRTSA_TRANSPORT_UDP = 0,
    HRTSA_TRANSPORT_WS = 1,
    HRTSA_TRANSPORT_UNITRANS_RELIABLE = 2,
    HRTSA_TRANSPORT_UNITRANS_UNRELIABLE = 3,
    HRTSA_TRANSPORT_MRIGHT_RELIABLE = 4,
    HRTSA_TRANSPORT_MRIGHT_UNRELIABLE = 5,
    HRTSA_TRANSPORT_NUM
};

struct HRTSAEncryptionConfig {
    HRTSACryptionMode cryptionMode;
    const char *cryptionSec;          // 只有cryptionMode==1，需要赋值
    HRTSASuiteType suiteType;         // 只有cryptionMode==1，需要赋值
    HRTSACryptionSecFormat secFormat; // 密钥的格式
};


struct HRTSALeaveStatsInfo {
    long long info = 0;
};

// ====================================video=======================================


struct HRTSAFrameVideoAnalysis {
    unsigned char layerId;   // 层id，0表示小P 1表示二层P  2表示大P 3表示I帧
    unsigned int refFrameTs; // 参考帧时间戳，使能扩展头带参考帧信息时使用
};

struct HRTSAFrameVideoOption {
    unsigned int uiWidth;                 /* *< 图像宽度 */
    unsigned int uiHeight;                /* *< 图像高度 */
    unsigned int timeSample;              // 编码时间戳
    HRTSAVideoFrameType frameType;        /* *< 帧类型 */
    HRTSAFrameVideoAnalysis analysisInfo; /* APK加密需要 */
    unsigned char *metaData;              /*用户私有信息*/
    unsigned int metaLen;                 /*用户私有信息长度       ,当前最大为15    */
    uint64_t globalTimeStamp;             /* 帧时间戳，可用于音画同步   */
};


struct HRTSAStreamInfo {
    HRTSAStreamType streamType; // 大流, 小流
    unsigned int bitrate;       // 分配的带宽
};

struct HRTSABandwidthDistribution {
    HRTSAStreamInfo streamInfo[RTSA_MAX_UP_STREAM_NUM];
};


struct UpNetworkVideoStatics {
    unsigned int avgdelay = 0;        /* RTT时延 */
    unsigned long long totalByte = 0; // 总发送字节数
    unsigned int totalPacketNum = 0;  // 总发包数
    unsigned int curSendBitRate = 0;  // 发送码率,包括FEC,NACK
    unsigned int estimateBitRate = 0; // 上行总预估带宽
    unsigned int streamNum = 0;       // 当前通道stream数量
};

struct DownNetworkVideoStatics {
    unsigned int netState = 0;         /* 接收网络状态 0-GOOD, 1-RISE, 2-HOLD, 5-REDUCE-5, 10-REUDCE-10 */
    unsigned int lostRate = 0;         /* 瞬时丢包率 */
    unsigned int longTermLossRate = 0; /* 长期丢包率 */
    unsigned int avgdelay = 0;         /* RTT时延 */
    unsigned int curRecvBitRate = 0;   /* 码率 */
    unsigned long long totalByte = 0;  // 总接收字节数
    unsigned int totalPacketNum = 0;   // 总发包数
    unsigned int estimateBitRate = 0;  // 下行总预估带宽
    unsigned int streamNum = 0;        // 当前通道stream数量
};

struct VideoSendStatics {
    char uid[RTSA_MAX_USERID_LEN];
    HRTSAStreamType streamType;
    unsigned int frameRate;                // 帧率
    unsigned int intralFrameOriginBitrate; // 参考帧码率(APK原始流码率，I +大P) *1.05(加上RTP+UDP大
    unsigned int mediaBitrate;             // 原始流码率*1.05(加上RTP+UDP大小)
    unsigned int networkBitrate;           // 实际发送的总带宽（包括FEC,NACK等）
    unsigned int estSendBitrate;           // 预估带宽 (total里面用户分出来的带宽)
    unsigned int encodeType;               // H264/H265编码
    unsigned int divideFrameDelay;         // 上行流入帧到拆包发出耗时（毫秒）
};

struct VideoRecvStatics {
    char uid[RTSA_MAX_USERID_LEN];
    HRTSAStreamType streamType;
    unsigned int frameRate;         // 帧率
    unsigned int mediaBitrate;      // 接收的原始流码率, 不包括FEC跟NACK
    unsigned int networkBitrate;    // 网络码率（包括FEC,NACK等）
    unsigned int estRecvBitRate;    // 预估带宽（SDK内部分配的toal，报给SFU的值）
    unsigned int totalFrozenTime;   // 远端用户在加入频道后发生视频卡顿的累计时长（毫秒）
    unsigned int dropDeltaFrameNum; // sfu丢弃的小P帧数量
    unsigned int intralFrameOriginBitrate; // sfu转发的I帧+大P帧速率
    unsigned int buildFrameDelay;   // 下行流收包到组帧上报耗时（毫秒）
};

struct VideoSendStaticsInfo {
    int streamNum;                                        // 统计流的路数
    VideoSendStatics sendStatics[RTSA_MAX_UP_STREAM_NUM]; // 统计流的详细信息
};

struct VideoRecvStaticsInfo {
    int streamNum;                                          // 统计流的路数
    VideoRecvStatics recvStatics[RTSA_MAX_DOWN_STREAM_NUM]; // 统计流的详细信息
};

// ====================================audio=======================================

struct HRTSAFrameAudioOption {
    unsigned int volume;          /* 音量  */
	uint64_t globalTimeStamp;     /* 帧时间戳，可用于音画同步   */
};

struct AudioRecvStaticsInfo {
    int rtt = 0;         // 端侧发送SR到SFU，SFU回RR到端侧，因此计算RTT需要在发送通道中
    int pktLoss = 0;     // 发送丢包率，对端RTCP反馈
    int jitter = 0;      // 抖动
    int pktReceived = 0; // 总收包数
    int pktLostCnt = 0;  // 总丢包数
    int recvBiteRate = 0;   //接收码率(网络流量)
};

struct AudioSendStaticsInfo {
    int rtt = 0;     // 端侧发送SR到SFU，SFU回RR到端侧，因此计算RTT需要在发送通道中
    int pktSent = 0; // 总发包数
    int pktLoss = 0; // 发送丢包率，对端RTCP反馈
    int sendBiteRate = 0;   //发送码率(网络流量)
};

struct HRTSAAudioMaxSpeaker {
    char userId[RTSA_MAX_USERID_LEN]; // 用户ID
    unsigned int volume;              // 音频数据能量值，范围[0,100]，0音量最小，100音量最大
};

struct HRTSAAudioMaxSpeakerInfo {
    int size;
    HRTSAAudioMaxSpeaker speaker[RTSA_MAX_SPEAKER_NUM];
};

// 用户信息
struct HRTSAUserInfo {
    char userId[RTSA_MAX_USERID_LEN] {0};
    char nickName[RTSA_MAX_USERNAME_LEN] {0};  // 用户昵称
    long long joinTime = 0; // 用户加入房间时间
};

struct HRTSAUserInfoList {
    int num = 0;
    HRTSAUserInfo users[RTSA_MAX_PAGE_NUM];
};

// ====================================cmd=======================================

// ====================================posix==================================
enum HRTSATransportPriority {
    HRTSA_STREAM_PRIORITY_0 = 0,            /*middle1*/
    HRTSA_STREAM_PRIORITY_1 = 1,            /*middle2*/
    HRTSA_STREAM_PRIORITY_2 = 2,            /*middle3*/
    HRTSA_STREAM_PRIORITY_3 = 3,            /*暂不支持*/
    HRTSA_STREAM_PRIORITY_4 = 4,            /*暂不支持*/
    HRTSA_STREAM_PRIORITY_5 = 5,            /*暂不支持*/
    HRTSA_STREAM_PRIORITY_6 = 6,            /*暂不支持*/
    HRTSA_STREAM_PRIORITY_7 = 7,            /*暂不支持*/
    HRTSA_STREAM_PRIORITY_MAX = 8,          
};

enum HRTSATransportErrorCode {
    HRTSA_TRANS_SUCCESS = 0,
    /* -101 ~ -200 自定义ErrorCode */
    HRTSA_TRANS_CONNECT_FAILED = -101,              // RTSA系统入会错误
    HRTSA_TRANS_CLOSE_FAILED   = -102,              // RTSA系统离会错误
    HRTSA_TRANS_EAGAIN  = -103,                     // EAGAIN错误
    HRTSA_TRANS_CONNECTION_RST = -104,              // 连接重置RST错误
    HRTSA_TRANS_STREAM_ALREADY_EXIST = -105,        // 该priority的stream已经创建
    HRTSA_TRANS_STREAM_NOT_EXIST = -106,			// 该priority的stream未创建
    HRTSA_TRANS_PEER_NOT_CONNECTED = -107,          // 对端未订阅成功
};

enum HRTSATransportLogLevel {
    HRTSA_CLIENT_ULOG_LEVEL_ERROR = 0,
    HRTSA_CLIENT_ULOG_LEVEL_WARN = 1,
    HRTSA_CLIENT_ULOG_LEVEL_INFO = 2,
    HRTSA_CLIENT_ULOG_LEVEL_DEBUG = 3,
};

typedef void(* IHRTSATransportLogCallback)(const HRTSATransportLogLevel level, const char *format, ...);

// Endpoint信息
struct HRTSAEndInfo {
    char userId[RTSA_MAX_USERID_LEN] {0};
};

struct HRTSAEndInfoList {
    int num = 0;
    HRTSAEndInfo end[RTSA_MAX_TRANSPORT_ENDPOINT_NUM];
};

struct HRTSASubscribeInfoList {
    HRTSAEndInfoList endInfoList[HRTSA_STREAM_PRIORITY_MAX];
};

struct HRTSATransportIoVec {
    void *iov_base;
    size_t iov_len = 0;
    HRTSATransportPriority priority; // 对应流的StreamType，每个StreamType底层都是唯一一个Stream
    char *userId; // 接收端有意义，发送端填nullptr
};

struct HRTSATransportMsgHdr {
    struct HRTSATransportIoVec *msg_iov;
    size_t msg_iovlen;
};

// 每100ms更新一次
struct HRTSATransportNetworkQuality {
    long long totalBytesSent;       // 单位Byte
    long long totalBytesSentByApp;  // 单位Byte
    long long totalBytesSentByRet;  // 单位Byte
    long long totalBytesSendByFec;  // 单位Byte
    long long totalBytesAcked;      // 单位Byte

    long long totalPacketSent;
    long long totalPacketLost;

    long long srtt;                 // 单位ms
    long long lastestRtt;           // 单位ms, 暂无

    long long lossRate;	            // 单位百分比，计算周期(100ms)的 min(totalPacketLost差值 / totalPacketSent差值, 100)
    long long retransRate;          // 单位百分比，计算周期(100ms)的 min(totalBytesSentByRet差值 / totalBytesSent差值, 100)
    long long spuriouslyRetransRate;// 单位百分比，计算周期(100ms)的 min(bytes_spuriously_retransmitted差值 / totalBytesSent差值, 100)

    long long sendRate;             // 单位bps，计算周期(100ms)的 totalBytesSent差值Byte * 8 / 100ms
    long long goodput;	            // 单位bps，计算周期(100ms)的 totalBytesAcked差值Byte * 8 / 100ms

    long long estimated_bandwidth;  // 单位bps
    long long encode_target_rate;   // 单位bps

    long long streamBytesWritten[HRTSA_STREAM_PRIORITY_MAX];    // 单位Byte
    long long streamBytesAcked[HRTSA_STREAM_PRIORITY_MAX];      // 单位Byte
    long long streamBytesBuffered[HRTSA_STREAM_PRIORITY_MAX];   // 单位Byte

    long long totalByteRecv;                                    // 单位Byte
    long long totalByteRecvByApp;                               // 单位Byte
    long long recvRate;                                         // 单位bps，计算周期(100ms)的 totalByteRecv差值Byte * 8 / 100ms
    long long totalPacketRecv;
    long long streamBytesRead[HRTSA_STREAM_PRIORITY_MAX];       // 单位Byte
};

struct HRTSAAudioEncStru {
    short *pcmInputData;        // 编码输入PCM数据指针
    unsigned int inputLength;   // 编码输入PCM数据长度，以样点为单位
    unsigned char *outputData;  // 编码输出编码后数据
    unsigned int outputLength;  // 编码输出编码后数据长度，以字节为单位
    unsigned int outputBufSize; // 编码数据输出缓冲区大小
};

struct HRTSAAudioDecStru {
    int bfi;                    // bad帧指示位 0：good帧 1：bad帧，丢包
    unsigned char* inputData;   // 解码输入码流指针
    unsigned int inputLength;   // 解码输入码流长度，以字节为单位，非传输帧时，长度为0
    short *outputData;          // 解码输出PCM数据指针
    unsigned int outputLength;  // 解码输出PCM数据长度，以样点为单位
    unsigned int outputBufSize; // 解码数据输出缓冲区大小
};

struct HRTSARtmpUrl {
    char url[RTSA_MAX_RTMP_URL_LEN + 1] {0};  // 地址
};

struct HRTSARtmpUrlList {
    int num = 0;
    HRTSARtmpUrl urlInfo[RTSA_MAX_RTMP_URL_NUM];  // 地址
};

struct HRTSARtmpUserInfo {
    // 用户ID
    char userId[RTSA_MAX_USERID_LEN]{0};
    // 视频大流
    bool main;
    // 视频小流
    bool slide;
    // 桌面流
    bool desktop;
    // 音频流
    bool audio;
};

struct HRTSARtmpConfig {
    // 旁路直播的输出视频的宽度，默认360（px）
    int width = 360;
    // 旁路直播的输出视频的高度，默认640（px）
    int height = 640;
    // 旁路直播的输出视频的码率，默认400（Kbps）
    int videoBitrate = 400;
    // 旁路直播的输出视频的帧率，默认15（fps）
    int videoFramerate = 15;
    // 旁路直播的输出视频的GOP，默认30帧
    int videoGop = 30;
    // 旁路直播的输出音频的采样率, 支持16000、32000、44100、48000 默认16000，
    int audioSampleRate = RTSA_AUDIO_SAMPLE_RATE_16KHZ;
    // 旁路直播的输出音频的码率，取值[1-128]Kbps，默认值为 48Kbps
    int audioBitrate = 48;
    // 旁路直播的输出音频的声道数, 默认单声道
    int audioChannels = 1;
    // 旁路直播的视频模式，0、悬浮，1、九宫格，2、屏幕分享
    int tmplate = 0;
};

struct HRTSATranscodeConfig {
    // 输出的rtmp流信息
    HRTSARtmpConfig config;
    // 合流用户数量
    int userNum = 0;
    // 合流用户信息
    HRTSARtmpUserInfo rtmpUsers[RTSA_MAX_RTMP_USER_NUM];
};

struct HRTSARtmpUrlInfo {
    char url[RTSA_MAX_RTMP_URL_LEN + 1] {0};
    int status;
    int errCode;
};

struct HRTSAUrlStatusList {
    int nSize = 0;
    HRTSARtmpUrlInfo rtmpUrlInfo[RTSA_MAX_RTMP_URL_NUM];
};

} // namespace rtsa
} // namespace huawei

#endif // RTSABUILD_HRTSAENGINEPARAM_H
