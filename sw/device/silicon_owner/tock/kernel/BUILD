# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//rules:linker.bzl", "ld_library")
load("//rules:opentitan.bzl", "elf_to_disassembly", "obj_transform")

package(default_visibility = ["//visibility:public"])

ld_library(
    name = "kernel_layout",
    includes = [
        "kernel_layout.ld",
    ],
)

ld_library(
    name = "layout",
    script = "layout.ld",
    deps = [
        ":kernel_layout",
    ],
)

cc_library(
    name = "nostartfiles",
    linkopts = [
        "-nostartfiles",
    ],
)

rust_binary(
    name = "kernel",
    srcs = [
        "@tock//boards/opentitan/earlgrey-cw310:src/io.rs",
        "@tock//boards/opentitan/earlgrey-cw310:src/otbn.rs",
        "@tock//boards/opentitan/earlgrey-cw310:src/main.rs",
    ],
    crate_root = "@tock//boards/opentitan/earlgrey-cw310:src/main.rs",
    rustc_flags = [
        "-g",
        #        "-Clinker=rust-lld",
        #        "-Clinker-flavor=ld.lld",
        #        "-Crelocation-model=static",
        #        "-Clink-arg=-nmagic",
        #        "-Clink-arg=-icf=all",
        #        "-Cforce-frame-pointers=no",
    ],
    deps = [
        ":layout",
        ":nostartfiles",
        "@tock//kernel",
	"@tock//arch/rv32i",
        "@tock//boards/components",
        "@tock//chips/earlgrey",
        "@tock//chips/lowrisc",
        "@tock//libraries/tock-tbf",
	#"@tock//capsules/aes_gcm:capsules-aes-gcm",
        "@tock//capsules/core:capsules-core",
        "@tock//capsules/extra:capsules-extra",
    ],
)

elf_to_disassembly(
    name = "kernel_dis",
    srcs = [":kernel"],
)

obj_transform(
    name = "raw_kernel",
    srcs = [":kernel"],
)
