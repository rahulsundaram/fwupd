/*
 * Copyright (C) 2022 Advanced Micro Devices Inc.
 * All rights reserved.
 *
 * This file is provided under a dual MIT/LGPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 * AMD Chooses the MIT license part of Dual MIT/LGPLv2 license agreement.
 *
 * SPDX-License-Identifier: LGPL-2.1+ OR MIT
 */

#pragma once

#include <fwupdplugin.h>

#define FU_TYPE_PCI_PSP_DEVICE (fu_pci_psp_device_get_type())
G_DECLARE_FINAL_TYPE(FuPciPspDevice, fu_pci_psp_device, FU, PCI_PSP_DEVICE, FuUdevDevice)
