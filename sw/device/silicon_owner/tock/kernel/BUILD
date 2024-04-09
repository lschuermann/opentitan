# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("//rules:linker.bzl", "ld_library")
load("//rules:opentitan.bzl", "OPENTITAN_CPU", "elf_to_disassembly", "obj_transform")
load("@//rules:rust.bzl", "crate_build")
load("@rules_rust//bindgen:bindgen.bzl", "rust_bindgen")

package(default_visibility = ["//visibility:public"])

ld_library(
    name = "kernel_layout",
    includes = [
        "@tock//boards:kernel_layout",
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

cc_library(
    name = "otcrypto_mac_hdrs",
    srcs = [
        "otcrypto_mac_hdrs.h",
    ],
    deps = [
        "//sw/device/lib/crypto/impl:mac",
    ],
)

rust_bindgen(
    name = "otcrypto_mac_bindings_rs",
    cc_lib = ":otcrypto_mac_hdrs",
    header = "otcrypto_mac_hdrs.h",
    bindgen_flags = [
        "--use-core",
    ],
)

genrule(
    name = "otcrypto_mac_bindings_src_rs",
    srcs = [":otcrypto_mac_bindings_rs"],
    outs = ["src/otcrypto_mac_bindings.rs"],
    cmd = "mkdir -p src && cp $< $@",
)

rust_binary(
    name = "kernel",
    srcs = [
        "src/io.rs",
        "src/main.rs",
        "src/otbn.rs",
	"src/ef_cryptolib_hmac.rs",
	"src/hmac_bench.rs",
	"src/otcrypto_mac_ef_bindings.rs",
	#":otcrypto_mac_bindings_src_rs",
        "src/pinmux_layout.rs",
    ],
    rustc_flags = [
        "-g",
        # TODO(opentitan#19491): determine the appropriate set of linker flags.
        #"-Clinker=rust-lld",
        #"-Clinker-flavor=ld.lld",
        #"-Crelocation-model=static",
        #"-Clink-arg=-nmagic",
        #"-Clink-arg=-icf=all",
        #"-Cforce-frame-pointers=no",
    ],
    # We specifically restrict our build target to the OpenTitan
    # CPU because tock does not support an x86_64 target.
    target_compatible_with = [OPENTITAN_CPU],
    deps = [
        ":layout",
        ":nostartfiles",
	"//sw/device/lib/crypto/drivers:hmac",
	"//sw/device/lib/crypto/impl:mac",
	"@tock//encapfn/encapfn",
	"@tock//encapfn-tock",
        "@tock//arch/rv32i",
        "@tock//boards/components",
        "@tock//capsules/aes_gcm:capsules-aes-gcm",
        "@tock//capsules/core:capsules-core",
        "@tock//capsules/extra:capsules-extra",
        "@tock//chips/earlgrey",
        "@tock//chips/lowrisc",
        "@tock//kernel",
        "@tock//libraries/tock-tbf",
    ],
    experimental_use_cc_common_link = 1,
)

obj_transform(
    name = "raw_kernel",
    srcs = [":kernel"],
    tags = ["manual"],
)
