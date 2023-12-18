/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: ����flavor����
 */
#ifndef RTSACONTROL_IHRTSATESTENV_H
#define RTSACONTROL_IHRTSATESTENV_H

#ifdef CONTROL_WINDOWS
#define RTSA_ENGINEAPI __declspec(dllexport)
#else
#define RTSA_ENGINEAPI __attribute__((__visibility__("default")))
#endif


/*
 * ���û�������
 * RTSA���ʵ�����ø�һ������������������
 * һ�����ã�����������Ч���������̽���ǰһֱ��Ч
 * ���л�����������������
 */
RTSA_ENGINEAPI void setFlavor(const char *env);

#endif // RTSACONTROL_IHRTSATESTENV_H
