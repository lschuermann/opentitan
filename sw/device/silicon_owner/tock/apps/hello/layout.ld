/* Copyright lowRISC contributors. */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */

/* Layout for the RISC-V 32 boards, used by the examples in this repository. */

TBF_HEADER_SIZE = 0x80;

FLASH_START = 0x20040000;
FLASH_LENGTH = 32M;

RAM_START = 0x10005000;
RAM_LENGTH = 512K;

INCLUDE sw/device/silicon_owner/tock/apps/hello/libtock_layout.ld
