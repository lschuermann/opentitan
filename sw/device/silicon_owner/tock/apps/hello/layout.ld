/* Copyright lowRISC contributors. */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */

/* Layout for the RISC-V 32 boards, used by the examples in this repository. */

TBF_HEADER_SIZE = 0x80;

MEMORY {
  /* Note that the SRAM address may need to be changed depending on
   * the kernel binary, check for the actual address of APP_MEMORY!
   */
  FLASH (X) : ORIGIN = 0x200a4000 + TBF_HEADER_SIZE, LENGTH = 32M - TBF_HEADER_SIZE
  RAM   (W) : ORIGIN = 0x10007000                  , LENGTH = 512K
}

INCLUDE sw/device/silicon_owner/tock/apps/hello/libtock_layout.ld
