# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# load("//rules:opentitan.bzl", "OPENTITAN_CPU")
load("//rules:linker.bzl", "ld_library")
load("//rules:tock.bzl", "tock_elf2tab")

package(default_visibility = ["//visibility:public"])

ld_library(
    name = "cmpsvc_layout",
    includes = [
        "cmpsvc_layout.ld",
    ],
)

ld_library(
    name = "layout",
    script = "layout.ld",
    deps = [
        ":cmpsvc_layout",
    ],
)

cc_binary(
    name = "cryptolib_cmpsvc_elf",
    srcs = [
        "main.c",
        "init_riscv32.S",
    ],
    deps = [
        ":layout",

        "//sw/device/lib/crypto/drivers:hmac",
        "//sw/device/lib/crypto/impl:mac",
    ],
    linkopts = [
        "-nostartfiles",
    ],
)

tock_elf2tab(
    name = "cryptolib_cmpsvc",
    src = ":cryptolib_cmpsvc_elf",
    disable = True,
    arch = "rv32imc",
)
