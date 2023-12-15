/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2012-2020. All rights reserved.
 * File name: basetype.h
 * Author & ID: 宋小刚+00133955 张丽萍+00139017 曹俊茂+00103648
 * Version: 1.00
 * Date:  2010-03-01
 * Description: 本文件包括了常用数据类型重定义的宏（windows、linux平台）
 * Function List:
 * History:
 * Revision 1.00  2010/03/01 10:00:00  songxg+00133955
 * 初始版本编码完成//Copyright (C), 1988-2018, Huawei Tech. Co., Ltd.
 *
 * evision 1.01  2010/03/02 14:15:00  songxg+00133955
 * 根据曹俊茂和张丽萍意见添加INT40、UINT40、BOOL类型及将防重定义宏去除
*/
#ifndef BASE_TYPE_H
#define BASE_TYPE_H

#ifdef __cplusplus
#if __cplusplus
extern "C" {
#endif    /* __cpluscplus */
#endif    /* __cpluscplus */

// 整数数据类型重定义
#if !defined(INT8)
typedef signed char        INT8;
#endif

#if !defined(INT16)
typedef signed short       INT16;
#endif
#if !defined(INT32)
typedef signed int         INT32;
#endif
#if !defined(UINT8)
typedef unsigned char      UINT8;
#endif
#if !defined(UINT16)
typedef unsigned short     UINT16;
#endif
#if !defined(UINT32)
typedef unsigned int       UINT32;
#endif

#if defined(__GNUC__)
#if !defined(INT64)
typedef          long long INT64;
#endif
#if !defined(UINT64)
typedef unsigned long long UINT64;
#endif
#if !defined(DOUBLE)
typedef double             DOUBLE;
#endif
#else
#if !defined(INT64)
typedef          __int64   INT64;
#endif
#if !defined(UINT64)
typedef unsigned __int64   UINT64;
#endif
#endif

// DSP可能会用到的数据类型
#if !defined(INT40)
typedef long               INT40;
#endif
#if !defined(UINT40)
typedef unsigned long      UINT40;
#endif

// 浮点数据类型重定义
#if !defined(FLOAT32)
typedef float              FLOAT32;
#endif
#if !defined(FLOAT64)
typedef double             FLOAT64;
#endif

// 空数据类型重定义
#if !defined(VOID)
typedef void               VOID;
#endif

// 布尔数据类型重定义
#if !defined(BOOL8)
typedef char               BOOL8;
#endif
#if !defined(BOOL16)
typedef short              BOOL16;
#endif
#if !defined(BOOL32)
typedef int                 BOOL32;
#endif
#if !defined(BOOL)
typedef int                 BOOL;
#endif

#ifndef TRUE
#define TRUE               1
#endif

#ifndef FALSE
#define FALSE              0
#endif

#ifdef __cplusplus
#if __cplusplus
}
#endif    /* __cpluscplus */
#endif    /* __cpluscplus */

#endif /* __BASE_TYPE_H__ */

