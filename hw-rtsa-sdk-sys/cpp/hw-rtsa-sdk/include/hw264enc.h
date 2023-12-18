/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2012-2020. All rights reserved.
Copyright (C), 1988-2019, Huawei Tech. Co., Ltd.
 * File name  : hw264enc_api.h
 * Author & ID: 何泽宇+00345560 叶挺峰+00381793 刘瑞欢+00412295 刘益群+00451292
 * Version    : 1.0
 * Date       :  2019-11
 * Description: HW264Enc新版接口文件
 * Function List:
 * History:
 * 1、2019-11 初始版本 何泽宇+00345560 叶挺峰+00381793 刘瑞欢+00412295 刘益群+00451292
*/

#ifndef HW264ENC_API_H
#define HW264ENC_API_H
#include <stdarg.h>
#include <stdint.h>
#include <stdlib.h>
#include "basetype.h"

/*  API调用定义 */
#ifdef HW264_API_IMPORTS
#define HW264_API __declspec(dllimport)
#else
#define HW264_API
#endif

#define H264E_HANDLE void *
#define H264E_MAX_NALU_IN_FRM 200
#define USER_VERSION_LENGTH 64
#define HW264E_VERSION_LENGTH 64 /*  length of version string */
#define HW264E_TIME_LENGTH 28    /*  length of time string */
#define HW264E_MAX_PLANE 3

#define H264E_OK 0
#define H264E_FAILED (-1)


/*  frameType类型 */
#define HW264_TYPE_AUTO 0x0000     /*  自动决策 */
#define HW264_TYPE_IDR 0x0001      /*  IDR帧 */
#define HW264_TYPE_I 0x0002        /*  I帧 */
#define HW264_TYPE_P 0x0003        /*  P帧 */
#define HW264_TYPE_BREF 0x0004     /*  B参考帧 */
#define HW264_TYPE_B 0x0005        /*  非B参考帧 */
#define HW264_TYPE_KEYFRAME 0x0006 /*  关键帧(I/IDR帧) */
#define IS_HW264_TYPE_I(x) ((x) == HW264_TYPE_I || (x) == HW264_TYPE_IDR || (x) == HW264_TYPE_KEYFRAME) /* 是否I帧 */
#define IS_HW264_TYPE_B(x) ((x) == HW264_TYPE_B || (x) == HW264_TYPE_BREF)                              /* 是否B帧 */


typedef enum H264E_CSP {
    H264E_YUV_420, //  vui显示范围limited range 16-235
    H264E_YUV_J420 //  vui显示范围full range 0-255
} H264E_CSP;

typedef enum H264E_ColorPrim {
    COLORPRIM_RESERVED,
    COLORPRIM_BT709,
    COLORPRIM_UNKNOWN,
    COLORPRIM_RESERVED2,
    COLORPRIM_BT470M,
    COLORPRIM_BT470BG,
    COLORPRIM_SMPTE170M,
    COLORPRIM_SMPTE240M,
    COLORPRIM_FILM,
    COLORPRIM_BT2020,
    COLORPRIM_SMPTE428,
    COLORPRIM_SMPTE431,
    COLORPRIM_SMPTE432
} H264E_ColorPrim;

typedef enum H264E_Transfer {
    TRANSFER_RESERVED,
    TRANSFER_BT709,
    TRANSFER_UNKNOWN,
    TRANSFER_RESERVED2,
    TRANSFER_BT470M,
    TRANSFER_BT470BG,
    TRANSFER_SMPTE170M,
    TRANSFER_SMPTE240M,
    TRANSFER_LINEAR,
    TRANSFER_LOG100,
    TRANSFER_LOG316,
    TRANSFER_IEC61966_2_4,
    TRANSFER_BT1361E,
    TRANSFER_IEC61966_2_1,
    TRANSFER_BT2020_10,
    TRANSFER_BT2020_12,
    TRANSFER_SMPTE2084,
    TRANSFER_SMPTE428,
    TRANSFER_ARIB_STD_B67
} H264E_Transfer;

typedef enum H264E_ColMatrix {
    COLMATRIX_GBR,
    COLMATRIX_BT709,
    COLMATRIX_UNKNOWN,
    COLMATRIX_RESERVED,
    COLMATRIX_FCC,
    COLMATRIX_BT470BG,
    COLMATRIX_SMPTE170M,
    COLMATRIX_SMPTE240M,
    COLMATRIX_YCGCO,
    COLMATRIX_BT2020NC,
    COLMATRIX_BT2020C,
    COLMATRIX_SMPTE2085,
    COLMATRIX_CHROMA_DERIVED_NC,
    COLMATRIX_CHROMA_DERIVED_C,
    COLMATRIX_ICTCP,
} H264E_ColMatrix;

typedef enum H264E_VideoFormat {
    VIDFORMAT_COMPONENT,
    VIDFORMAT_PAL,
    VIDFORMAT_NTSC,
    VIDFORMAT_SECAM,
    VIDFORMAT_MAC,
    VIDFORMAT_UNKNOWN,
} H264E_VideoFormat;

typedef enum H264E_PvcStrength {
    PVC_SUR_100 = 0,
    PVC_SUR_70
} H264E_PvcStrength;

typedef enum H264E_PvcWeak {
    PVC_WEAK_OFF = 0,
    PVC_WEAK_ON
} H264E_PvcWeak;
typedef enum H264E_PvcMode {
    PVC_MODE_PHONE = 0,
    PVC_MODE_TV
} H264E_PvcMode;

typedef enum H264E_THREADMODE {
    FRAME_THREAD = 0,
    SLICE_THREAD = 1
} H264E_THREADMODE;

typedef enum H264E_Preset {
    H264E_PRESET_LIVE_AUTO = 0, // auto
    H264E_PRESET_Q2 = 2,        // slower
    H264E_PRESET_Q3 = 3,        // slow
    H264E_PRESET_Q5 = 5,        // medium
    H264E_PRESET_Q6 = 6,        // fast
    H264E_PRESET_Q7 = 7         // faster
} H264E_Preset;

typedef enum H264E_Profile {
    PROFILE_BASELINE = 66, /* baseline */
    PROFILE_MAIN = 77,     /* main */
    PROFILE_HIGH = 100,    /* high */
} H264E_Profile;
typedef enum H264E_PvcLevel {
    PVC_NONE = 0, /* 关闭pvc */
    PVC_V1,       /* pvc版本1.0 */
    PVC_V2        /* pvc版本2.0 */
} H264E_PvcLevel;

// 质量增强模式
typedef enum H264E_QualEnhMode {
    QE_OFF = 0,                  // 关闭质量增强
    QE_SHARP,                    // 锐化
    QE_CONTRAST_LIMIT_SHARP,     // 对比度限制锐化，牺牲部分锐利度以保护高清字幕等易出白边区域
    QE_VMAF,                     // 提升 VMAF，假定 VMAF 通过与同分辨率源对比得出
    QE_UPSCALED_VMAF             // 提升 VMAF，假定 VMAF 通过上采样后与高分辨源对比得出
} H264E_QualEnhMode;

typedef enum H264E_RcMode {
    H264E_RC_ABR, // ABR模式,固定码率但允许一定码率波动
    H264E_RC_CRF, // 固定CRF模式
    H264E_RC_CQP, // fixed QP mode
    H264E_RC_TRC, // transmission controlled mode
    H264E_RC_CCRF, // clamped CRF
} H264E_RcMode;

/* 客观质量调优选项            */
typedef enum H264E_Tune {
    SSIM = 0,
    VMAF = 1,
    PSNR = 2,
    NOISE = 3
} H264E_Tune;

typedef enum H264E_ParCovStrength {
    UNLIMITED = 0, /*  不约束峰值码率收敛的速度 */
    WEAK,          /*  峰值码率收敛速度慢 */
    MEDIUM,        /*  峰值码率收敛速度中等 */
    STRONG         /*  峰值码率收敛速度快 */
} H264E_ParCovStrength;

typedef enum H264E_DynamicBitrateAdjustMode {
    DYNAMICRATE_USERS_MODE     = 0,     /* 按照下发固定码率编码 */
    DYNAMICRATE_LOWER_MODE     = 1,     /* 以min(下发码率， 源码率)编码 */
    DYNAMICRATE_INPUT_MODE     = 2      /* 以源码率编码 */
} H264E_DynamicBitrateAdjustMode;

typedef enum H264E_NalUType {
    H264E_NAL_UNKNOWN = 0,
    H264E_NAL_SLICE = 1, /*  slice */
    H264E_NAL_SLICE_DPA = 2,
    H264E_NAL_SLICE_DPB = 3,
    H264E_NAL_SLICE_DPC = 4,
    H264E_NAL_SLICE_IDR = 5, /*  IDR Ref_IDC is non-zero */
    H264E_NAL_SEI = 6,       /*  Ref_IDC is 0 */
    H264E_NAL_SPS = 7,       /*  SPS */
    H264E_NAL_PPS = 8,       /*  PPS */
    H264E_NAL_AUD = 9,
    H264E_NAL_FILLER = 12, /*  填充 */
                           /*  Ref_IDC == 0 for 6,9,10,11,12 */
} H264E_NalUType;


typedef struct H264E_ShareBuff {
    UINT64 size; // buff大小
    UINT8 *addr;
} H264E_ShareBuff;


typedef struct HW264EncInArg {
    INT32 inFrameType;     /* 输入视频当前帧类型 */
    INT32 periodBitrate1s; /* 输入视频1s的码率 */
    INT64 pts;             /* 输入帧pts */
    INT32 inFrameSize;     /* 输入视频当前帧大小 */
    BOOL8 isForceIDRFrame; /* 是否强制当前帧为IDR帧 */
    INT32 roiMapSize;      /* ROI区域权重值MAP数据的个数，要求roiMapSize = width * height */
    FLOAT32 *roiWeightMap; /* ROI区域权重值MAP数据，权重值范围[0.0, 1.0] */
} HW264EncInArg;

typedef struct HW264EncInData {
    UINT32 stride[HW264E_MAX_PLANE];    /* 存放YUV数据buffer的宽度 */
    UINT32 pixelSize[HW264E_MAX_PLANE]; /* YUV数据空间大小 */
    UINT8 *pixelAddr[HW264E_MAX_PLANE]; /* YUV数据 */
} HW264EncInData;

typedef struct HW264EncInput {
    HW264EncInArg inArg;   /* 输出参数结构体 */
    HW264EncInData inData; /* 输出数据结构体 */
} HW264EncInput;

typedef struct HW264EncNal {
    INT32 naluSize;          /* NALU的长度 */
    UINT8 *naluAddr;         /* NALU数据 */
    H264E_NalUType naluType; /* NALU类型 */
} HW264EncNal;

typedef struct HW264EncOutArg {
    INT32 outFrameType;  /* 输出帧类型 */
    INT32 outFramePoc;   /* 输出POC值 */
    BOOL8 fixedKeyFrame; /* 输出是否为固定间隔I帧 */
    INT32 pts;   /* 输出PTS值 */
} HW264EncOutArg;
typedef struct HW264EncOutData {
    INT32 dataSize;                              /* 一帧码流的长度. */
    UINT8 *dataAddr;                             /* 输出一帧码流的 */
    INT32 nalNum;                                /* 一帧码流里NAL单元的个数. */
    HW264EncNal nalu[H264E_MAX_NALU_IN_FRM + 1]; /* 一帧码流里每个NAL单元的信息 */
} HW264EncOutData;
typedef struct HW264EncOutput {
    HW264EncOutArg outArg;   /* 输出参数结构体 */
    HW264EncOutData outData; /* 输出数据结构体 */
} HW264EncOutput;


typedef struct Tag264EVersion {
    INT8 versionChar[HW264E_VERSION_LENGTH]; /*  library version */
    INT8 releaseTime[HW264E_TIME_LENGTH];    /*  time */
    UINT32 compilerVersion;                   /* 编译器版本 */
} HW264_VERSION_STRU, *HW264_VERSION_PST;

/* *****************************************************************************
 * 回调注册函数名: H264E_MALLOC_FXN
 * 函数功能描述:  内存申请函数
 * 参数说明: channelIdx  - 编码器索引; len       - 申请内存长度
 * **************************************************************************** */
typedef VOID *(*H264E_MALLOC_FXN)(INT32 channelIdx, size_t len);

/* *****************************************************************************
 * 回调注册函数名:H264E_FREE_FXN
 * 函数功能描述: 内存释放函数
 * 参数说明: channelIdx  - 编码器索引; addr         - 源内存
 * **************************************************************************** */
typedef void (*H264E_FREE_FXN)(INT32 channelIdx, VOID *addr);

/* *****************************************************************************
 * 回调注册函数名:H264E_REALLOC_FXN
 * 函数功能描述: 内存重申请函数
 * 参数说明: channelIdx  - 编码器索引; add         - 源内存; len       - 申请内存长度
 * **************************************************************************** */
typedef void *(*H264E_REALLOC_FXN)(INT32 channelIdx, VOID *add, size_t len);
/* *****************************************************************************
 * 回调注册函数名:H264E_LOG_FXN
 * 函数功能描述: 日志打印函数
 * 参数说明: channelIdx   - 编码器索引
 * fileName     - 打印位置文件名
 * line      - 打印位置行数
 * outHandle     - 外部信息句柄
 * level      - 日志等级
 * msg          - 日志信息
 * **************************************************************************** */
typedef void (*H264E_LOG_FXN)(INT32 channelIdx, const INT8 *fileName, UINT32 line, VOID *outHandle, INT32 level,
    const INT8 *msg, va_list);
/* *****************************************************************************
 * 回调注册函数名:HW264_OUTPUT_FRAME_FXN
 * 函数功能描述: 编码器码率输出回调函数
 * 参数说明: channelIdx  - 编码器索引
 * handle       - 外部信息句柄
 * outputData   - 编码器输出信息结构体
 * **************************************************************************** */
typedef void *(*HW264_OUTPUT_FRAME_FXN)(INT32 channelIdx, VOID *handle, HW264EncOutput *outputData);


typedef struct HW264E_ThreadParam {
    H264E_THREADMODE threadMode; /* 0:帧级多线程 1：slice级多线程 */
    UINT32 frameThreadNum;       /* 帧级线程数,范围[0, 64] */
    UINT32 lookaheadThreadNum;   /* lookahead线程数,[0, 16] */
} H264E_ThreadParam;
typedef enum H264E_DbgInfo {
    HW264_LOG_NONE = 0,    /* 关闭日志 */
    HW264_LOG_ERROR = 1,   /* 打开error级别，只上报错误 */
    HW264_LOG_WARNING = 2, /* 打开warning级别 */
    HW264_LOG_INFO = 3,    /* 打开info级别，上报重要消息 */
    HW264_LOG_DEBUG = 4    /* 打开debug级别 */
} H264E_DbgInfo;
typedef struct H264ECallBackFunc {
    H264E_LOG_FXN logFunc;                  /* 用户传入的编码器日志回调函数 */
    VOID *logHanle;                         /* 编码器日志输出路径 */
    H264E_DbgInfo dbgInfo;                  /* 编码器日志输出等级，详细请看H264E_eDbgInfo */
    H264E_MALLOC_FXN mallocFunc;            /* 内存申请函数，取值要求非NULL */
    H264E_REALLOC_FXN reallocFunc;          /* 内存复申请函数，取值要求非NULL */
    H264E_FREE_FXN freeFunc;                /* free函数，取值要求非NULL */
    HW264_OUTPUT_FRAME_FXN outputFrameFunc; /* 编码输出一帧数据回调函数，不能为NULL */
    VOID *outHandle;                        /*  外部句柄用于回调函数的入参 */
} H264ECallBackFunc;

#define H264E_PAD_WIDTH (64 + 16)
#define H264E_PAD_HEIGHT (64 + 16)
#define H264E_PIC_STRIDE_ALIGN 64
#define H264E_ALIGN(x, a) (((x) + ((a) - 1)) & ~((a) - 1))

typedef enum H264E_SharpType {
    H264E_PSHARP_OFF = 0,     // Do not apply sharpening of the input frame before encoding
    H264E_PSHARP_SA3x3 = 3,    // Apply 3x3 sharpening with given weight as separate standalone pass before encoder
    H264E_PSHARP_SA5x5 = 5,    // Apply 5x5 sharpening with given weight as separate standalone pass before encoder
    H264E_PSHARP_SA7x7 = 7,    // Apply 7x7 sharpening with given weight as separate standalone pass before encoder
} H264E_SharpType;

typedef struct H264E_SharpParam{
    H264E_SharpType preSharpenType;   // Type of sharpening filter
    float preSharpenWeight;           // Explicit sharpening strength weight (for suported filtering types)
}H264E_SharpParam;

typedef struct H264E_EncodeParams {
    /*  编码器索引，取值范围[0, 1023]           */
    INT32 channelIdx;

    /*  输入图像宽高，支持横屏竖屏分辨率， 长边取值范围[32,8192], 短边取值范围[32,4320] */
    INT32 width;
    INT32 height;

    /*  输入图像的色彩空间，H264E_YUV_420/H264E_YUV_J420 */
    H264E_CSP colorSpace;

    /* 输入图像的RGB空间对应的绝对颜色XYZ的变换，默认值为UNKNOWN，详见枚举类型H264E_ColorPrim */
    H264E_ColorPrim colorPrimary;

    /* 输入图像的transfer funciton的gamma值，默认值为UNKNOWN，详见枚举类型H264E_Transfer */
    H264E_Transfer transfer;

    /* 输入图像的从RGB原色导出的亮度和色度信号矩阵系数，默认值为UNKNOWN，详见枚举类型H264E_ColMatrix */
    H264E_ColMatrix colMatrix;

    /* 输入图像的视频格式，默认值为5，详见枚举类型H264E_VideoFormat */
    H264E_VideoFormat videoFormat;

    /* 样点高宽比的水平和垂直尺寸，应为互质或等于0 */
    INT32 sarWidth;
    INT32 sarHeight;

    /*  连续b帧个数，取值范围[-1 ,7], -1: 推荐默认值(7); 当时延不为0时，连续b帧个数会被编码器刷新 */
    INT32 bframes;

    /*  b帧分层，取值范围[-1, 0, 1], -1:推荐默认值(1)；0：B帧不作为参考帧； 1：B帧做参考帧 */
    INT32 bframeRef;

    /*  帧率，与ffmpeg中的表示方法保持一致的，取值范围 fpsNum/fpsDen: (0,120] */
    INT32 fpsNum;
    INT32 fpsDen;

    /*  最大I帧间隔，取值范围[0,1<<30], 0: 推荐默认值125 */
    INT32 maxIDRFrameIntervel;

    /*  转码pass值，取值范围{0,1,2}, 0: singlpass编码模式；1：2pass编码模式的第1遍编码； 2：2pass编码模式的第2遍编码 */
    INT32 pass;

    /*  是否为变帧率输入，取值范围[0,1]，变帧率输入模式下，必须配置时间戳Timebase */
    BOOL32 vfrInput;

    /*  时间戳Timebase，变帧率输入模式生效，取值范围 timeBaseDen/timeBaseNum >= fpsNum/fpsDen */
    INT32 timeBaseNum;
    INT32 timeBaseDen;

    /* 编码工具级别 */
    H264E_Profile profile;

    /*  编码质量级别，取值范围详见枚举H264E_Preset，Q2到Q7档位，编码速度增快，编码质量降低，
   H264E_PRESET_LIVE_AUTO为直播自适应档位，根据设定的时延、宽高、帧率和码率，选择Q2到Q7中质量最好并且可实时的档位，
   设置H264E_PRESET_LIVE_AUTO档位时，时延delayTime不开为0 */
    H264E_Preset preset;

    /*  编码调控倾向，详见枚举H264E_Tune 商用只支持0（SSIM） */
    H264E_Tune tune;

    /*  码控模式，H264E_RC_ABR:码率控制模式，需设定目标码率bitRate；H264E_RC_CRF：质量控制模式：需设定输出质量等级crf */
    H264E_RcMode rcMode;

    /*  目标码率，取值范围[10,800000]kbps */
    INT32 bitRate;

    /*  质量等级，取值范围[0,51]，数值越小质量越高 */
    FLOAT32 crf;

    /*  时延，取值范围{0,400,1000}ms，0：不控制时延；400/1000:
   调节连续b帧个数、缓存帧数、线程配置自适应I帧等参数，
   将时延控制在400ms/1000ms左右，需要和H264E_PRESET_LIVE_AUTO配合使用 */
    INT32 delayTime;

    /*  编码器缓存帧数，取值范围[bframes,150], 用于帧类型分析和码控分析, 当时延不为0时，数值会被编码器刷新 */
    INT32 delayFrameNum;

    /*  是否开启场景切换自适应插入I帧，取值范围[0,1]，场景切换插入I帧后，I帧间隔计数默认刷新，当时延不为0时，数值会被编码器刷新
     */
    BOOL32 adaptiveIDRframe;

    /*  是否固定间隔必有I帧，取值范围[0,1]，开启自适应I帧后此开关有效，开启时，场景切换插入I帧后，I帧间隔计数不刷新，
   配置的最大I帧间隔出必会为I帧，开启时，HW264Enc_OutArg中的fixedKeyFrame标识输出帧是否为固定间隔的I帧 */
    BOOL32 fixedIntervalIDRframe;

    /*  峰值码率和平均码率的比值，取值范围[1,20] */
    FLOAT32 bitRatePar;

    /* 码率调整模式，取值范围[0,1,2] */
    H264E_DynamicBitrateAdjustMode dynamicBitrateMode;

    /*  峰值码率收敛到平均码率的强度，取值范围详见枚举H264E_ParCovStrength */
    H264E_ParCovStrength bitrateParCovStrength;

    /*  编码线程控制参数结构体，具体成员变量详见H264E_ThreadParam注释说明 */
    H264E_ThreadParam threadParam;

    H264E_PvcMode pvcMode;

    /*  感知编码等级，取值范围{0,1,2}，0: 关闭感知编码；1：旧的V1版本，没有基于内容的调节；2：v2版本，加入基于内容的感知
   模型。感知编码开启后，将设定的目标码率标准转码的输出作为参考的主观质量水平，在保证主观质量不下降的情况下节省码率。
   感知编码开启时，目标码率可为0，表示将源视频作为参考的主观质量水平。                                    */
    H264E_PvcLevel pvcLevel;

    /*  感知编码强度，取值范围{0,1}，0: 主观质量不变满足所有测试者的认可； 1： 主观质量不变满足大部分测试者的认可，
   码率节省更多 */
    H264E_PvcStrength pvcStrength;

     /*  感知编码弱化，取值范围{0,1}，0: 普通高清低码； 1：根据时域复杂度微调版本dqp<=1 */
    H264E_PvcWeak pvcWeak;

    /* ROI区域QP调节强度, 根据每帧图像输入参数中ROI数据对QP进行调节， 0为不调节，数值越大，ROI区域的QP值越小, */
    FLOAT32 roiQpOffsetStrenght;

    /*  质量增强相关参数 */
    H264E_QualEnhMode qualEnhMode;       // 质量增强模式，详见 H264E_QualEnhParam 注释说明
    UINT32 qualEnhLevel;                 // 质量增强等级，取值范围 [0, 8]，取 0 等价于 qualEnhMode = QE_OFF
    UINT32 qualEnhThreadNum;             // 质量增强所用线程数，取值范围 [0, 128]，取 0 等价于 qualEnhMode = QE_OFF

    H264E_ShareBuff *frameStatInfo;
    H264E_ShareBuff *blockStatInfo;

    /*  回调函数结构体参数 */
    H264ECallBackFunc callBackFunc;

    /*  下发上层调用版本号写入码流，例如：iMedia 1.0.Cloud0.B010 */
    UINT8 version[USER_VERSION_LENGTH];
    H264E_SharpParam sharpParam;
} H264E_EncodeParams;

/* *****************************************************************************
 * 函数名:  H264E_EncoderHeaders
 * 函数功能描述: 写入sps\pps参数.
 * 参数说明: hEncInfo - 编码器句柄;  nalAdr - nal结构体; nalNum - nan数量
 * 返回值: 成功：编码header的size  失败：-1
 * ***************************************************************************** */
INT32 H264E_EncoderHeaders(H264E_HANDLE hEncInfo, HW264EncNal *nalAdr, INT32 *nalNum);


/* *****************************************************************************
 * 函数名:  H264E_EncoderOpen
 * 函数功能描述: 创建编码器句柄
 * 参数说明: encodeParams   - 编码器参数结构体
 * 返回值: 成功：编码器句柄  失败： NULL
 * ***************************************************************************** */
H264E_HANDLE H264E_EncoderOpen(H264E_EncodeParams *encodeParams);

/* *****************************************************************************
 * 函数名:  H264E_EncoderProcess
 * 函数功能描述: 编码器处理一帧的输入数据
 * 参数说明: hEncInfo        - 编码器句柄; inputData       - 输入数据结构体
 * 返回值:  成功：0 失败：-1
 * ***************************************************************************** */
INT32 H264E_EncoderProcess(H264E_HANDLE hEncInfo, HW264EncInput *inputData);
/* *****************************************************************************
 * 函数名:  H264E_EncoderDelete
 * 函数功能描述: 关闭编码器
 * 参数说明: hEncInfo        - 编码器句柄;
 * 返回值: 成功：0  失败：-1
 * ***************************************************************************** */
INT32 H264E_EncoderDelete(H264E_HANDLE hEncInfo);


/* *****************************************************************************
 * 函数名:  编码信息内存创建函数
 * 函数功能描述: 2pass模式，申请帧级信息和块级信息的编码器共享内存
 * 参数说明: uiChannelID       - 编码器索引
 * frameStatInfo         - 帧级信息内存信息
 * blockStatInfo       - 块级信息内存信息
 * mallocFunc        - 内存申请函数
 * freeFunc          - 内存释放函数
 * width             - 图像宽，用于计算申请内存的大小
 * height            - 图像高，用于计算申请内存的大小
 * frameNum          - 图像帧数，用于计算申请内存的大小
 * 返回值: 成功：0  失败：非0
 * ***************************************************************************** */
INT32 H264E_CreatepasslogInfo(INT32 channelIdx, H264E_ShareBuff **frameStatInfo, H264E_ShareBuff **blockStatInfo,
    H264E_MALLOC_FXN mallocFunc, H264E_FREE_FXN freeFunc, INT32 width, INT32 height, INT32 frameNum);
/* *****************************************************************************
 * 函数名:  编码信息内存清理函数
 * 函数功能描述: 2pass模式，释放帧级信息和块级信息的编码器共享内存
 * 参数说明: channelIdx       - 编码器索引
 * frameStatInfo     - 帧级信息内存信息
 * blockStatInfo     - 块级信息内存信息
 * freeFunc          - 内存释放函数
 * 返回值: 成功：0  失败：非0
 * ***************************************************************************** */
INT32 H264E_DestorypasslogInfo(INT32 channelIdx, H264E_ShareBuff *frameStatInfo, H264E_ShareBuff *blockStatInfo,
    H264E_FREE_FXN freeFunc);
/* *****************************************************************************
 * 函数名:  H264E_GetBlockStatInfo
 * 函数功能描述: 2pass模式，释放帧级信息和块级信息的编码器共享内存
 * 参数说明: hEncObj           - 编码器句柄
 * blockStatInfo     - 块级信息内存信息
 * 返回值: 成功：0  失败：非0
 * ***************************************************************************** */
INT32 H264E_GetBlockStatInfo(H264E_HANDLE hEncObj, H264E_ShareBuff *blockStatInfo);

/* *****************************************************************************
 * 函数名:  H264E_GetVersion
 * 函数功能描述: 获取编码器版本相关信息
 * 参数说明: version         - 版本信息结构体
 * 返回值: version结构体
 * ***************************************************************************** */
INT32 H264E_GetVersion(HW264_VERSION_STRU *version);


#endif
