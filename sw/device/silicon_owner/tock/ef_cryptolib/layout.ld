/* Copyright lowRISC contributors. */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */

TBF_HEADER_SIZE = 0x90;

FLASH_START  = 0x20040000;
FLASH_LENGTH = 0x00008000;
RAM_START    = 0x10010000;
RAM_LENGTH   = 0x00010000;

/* Stack size, including values stacked by the Rust runtime */
STACK_SIZE   = 24 * 1024;

INCLUDE sw/device/silicon_owner/tock/ef_cryptolib/encapfn_tbf_layout.ld
