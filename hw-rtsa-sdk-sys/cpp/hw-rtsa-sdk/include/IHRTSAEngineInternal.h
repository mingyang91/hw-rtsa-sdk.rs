/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: hw rtsa sdk
 */

#ifndef RTSACONTROL_IRTSAENGINEINTERNAL_H
#define RTSACONTROL_IRTSAENGINEINTERNAL_H

#include "IHRTSAEngine.h"
#include "HRTSAEngineParam.h"
#include "IHRTSAEventHandlerInternal.h"


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
class IHRTSAEngineInternal: public IHRTSAEngine {
public:
    virtual ~IHRTSAEngineInternal() = default;
    // ==================================Common==============================================
    /* *
     * 设置信令相关信息字符串
     * @return 0(success),other(failed)
     */
    virtual int sendMsg(const char* msgInfo) = 0;

    virtual int getLocalConnIdByUsrUid() = 0;

    virtual int getRemoteConnIdByUsrUid(const char* usrid) = 0;

    virtual int setVersion(const char *version) = 0;
};
} // namespace rtsa
} // namespace huawei

RTSA_ENGINEAPI huawei::rtsa::IHRTSAEngineInternal *createHRTSAEngineInternal(huawei::rtsa::HRTSACreateParam *createParam,
    huawei::rtsa::IHRTSAEventHandlerInternal *rtsaEventHandler);


#endif // RTSACONTROL_IRTSAENGINEINTERNAL_H
