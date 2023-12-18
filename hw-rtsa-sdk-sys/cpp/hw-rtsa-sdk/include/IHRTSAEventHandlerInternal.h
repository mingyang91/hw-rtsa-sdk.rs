//
// Created by g00522473 on 2021/11/27.
//

#ifndef RTSABUILD_IHRTSAEVENTHANDLERINTERNAL_H
#define RTSABUILD_IHRTSAEVENTHANDLERINTERNAL_H

#include "IHRTSAEventHandler.h"
#include "HRTSAEngineParam.h"
namespace huawei {
namespace rtsa {
class IHRTSAEventHandlerInternal: public IHRTSAEventHandler {
public:
    // ==================================Common==============================================
    virtual ~IHRTSAEventHandlerInternal() = default;
 
    /**
     * 动态配置信息回调
     */
    virtual void onConfigNotify(const char* cfgInfo) { }

     /**
     * 信令相关信息回调
     */
    virtual void onRecvMsg(const char* userId, const char* msgInfo) { }

    /*设置用户Rid*/
    virtual void onJoinRoomSuccessSetRid(const uint64_t rid) {} 
};
} // namespace rtsa
} // namespace huawei

#endif // RTSABUILD_IHRTSAEVENTHANDLERINTERNAL_H
