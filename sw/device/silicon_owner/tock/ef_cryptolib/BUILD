# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# load("//rules:opentitan.bzl", "OPENTITAN_CPU")
load("//rules:linker.bzl", "ld_library")
load("//rules:tock.bzl", "tock_elf2tab")

package(default_visibility = ["//visibility:public"])

ld_library(
    name = "encapfn_tbf_layout",
    includes = [
        "encapfn_tbf_layout.ld",
	#"@tock//encapfn-tock:encapfn_c_rt/encapfn_layout.ld",
    ],
)

ld_library(
    name = "layout",
    script = "layout.ld",
    deps = [
        ":encapfn_tbf_layout",
    ],
)

cc_binary(
    name = "ef_cryptolib_elf",
    srcs = [
        "main.c",
        #"init_riscv32.S",
	"@tock//encapfn-tock:encapfn_c_rt/init_riscv32.S",
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
    name = "ef_cryptolib",
    src = ":ef_cryptolib_elf",
    disable = True,
    arch = "rv32imc",
)
