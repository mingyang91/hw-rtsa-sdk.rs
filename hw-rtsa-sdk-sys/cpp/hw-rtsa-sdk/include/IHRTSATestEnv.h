/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: 设置flavor环境
 */
#ifndef RTSACONTROL_IHRTSATESTENV_H
#define RTSACONTROL_IHRTSATESTENV_H

#ifdef CONTROL_WINDOWS
#define RTSA_ENGINEAPI __declspec(dllexport)
#else
#define RTSA_ENGINEAPI __attribute__((__visibility__("default")))
#endif


/*
 * 设置环境参数
 * RTSA多个实例共用该一个参数，请慎重设置
 * 一旦设置，整个进程生效。即，进程结束前一直生效
 * 若切换环境，请重新设置
 */
RTSA_ENGINEAPI void setFlavor(const char *env);

#endif // RTSACONTROL_IHRTSATESTENV_H
