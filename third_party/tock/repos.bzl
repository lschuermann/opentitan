# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("@//rules:repo.bzl", "bare_repository", "http_archive_or_local")
load("@//rules:rust.bzl", "crate_build")

def tock_repos(tock = None, libtock = None, elf2tab = None):
    bare_repository(
        name = "tock",
        local = tock,
        strip_prefix = "tock-45bacead310cd627958dc9ecc2fc4c4d4ca5e66a",
        url = "https://github.com/lowRISC/tock/archive/45bacead310cd627958dc9ecc2fc4c4d4ca5e66a.tar.gz",
        sha256 = "e4f7bbb1f672f0d9630ea2c1c0d5ccc9dd68e03f694cf4baa4cbf3173685334c",
        additional_files_content = {
            "BUILD": """exports_files(glob(["**"]))""",
            "arch/riscv/BUILD": crate_build(
                name = "riscv",
                deps = [
                    "//kernel",
                    "//libraries/tock-register-interface:tock-registers",
                    "//libraries/riscv-csr",
                ],
            ),
            "arch/rv32i/BUILD": crate_build(
                name = "rv32i",
                deps = [
                    "//arch/riscv",
                    "//kernel",
                    "//libraries/tock-register-interface:tock-registers",
                    "//libraries/riscv-csr",
                ],
            ),
            "boards/components/BUILD": crate_build(
                name = "components",
                deps = [
                    "//kernel",
                    "//capsules/core:capsules-core",
                    "//capsules/extra:capsules-extra",
                ],
            ),
            "capsules/core/BUILD": crate_build(
                name = "capsules-core",
                deps = [
                    "//kernel",
                    "//libraries/enum_primitive",
                    "//libraries/tickv",
                ],
            ),
            "capsules/extra/BUILD": crate_build(
                name = "capsules-extra",
                deps = [
                    "//kernel",
                    "//libraries/enum_primitive",
                    "//libraries/tickv",
                    "//capsules/core:capsules-core",
                ],
            ),
            "chips/earlgrey/BUILD": crate_build(
                name = "earlgrey",
                deps = [
                    "//chips/lowrisc",
                    "//arch/rv32i",
                    "//kernel",
                ],
            ),
            "chips/lowrisc/BUILD": crate_build(
                name = "lowrisc",
                deps = [
                    "//arch/rv32i",
                    "//kernel",
                ],
            ),
            "contsvc/BUILD": crate_build(
                name = "contsvc",
                deps = [
                    "//kernel",
                    "//libraries/tock-tbf",
                ],
            ),
            "libraries/enum_primitive/BUILD": crate_build(
                name = "enum_primitive",
            ),
            "libraries/riscv-csr/BUILD": crate_build(
                name = "riscv-csr",
                deps = [
                    "//libraries/tock-register-interface:tock-registers",
                ],
            ),
            "libraries/tickv/BUILD": crate_build(
                name = "tickv",
            ),
            "libraries/tock-cells/BUILD": crate_build(
                name = "tock-cells",
            ),
            "libraries/tock-tbf/BUILD": crate_build(
                name = "tock-tbf",
            ),
            "libraries/tock-register-interface/BUILD": crate_build(
                name = "tock-registers",
                crate_features = [
                    "default",
                    "register_types",
                ],
            ),
            "kernel/BUILD": crate_build(
                name = "kernel",
                deps = [
                    "//libraries/tock-register-interface:tock-registers",
                    "//libraries/tock-cells",
                    "//libraries/tock-tbf",
                ],
            ),
            "boards/opentitan/earlgrey-cw310/BUILD": crate_build(
                name = "earlgrey-cw310",
                deps = [
                    "//arch/rv32i",
                    "//boards/components",
                    "//kernel",
                    "//chips/earlgrey",
                    "//chips/lowrisc",
                    "//libraries/tock-tbf",
                    "//capsules/core:capsules-core",
                    "//capsules/extra:capsules-extra",
                    "//contsvc",
                ],
            ),
        },
    )

    bare_repository(
        name = "libtock",
        local = libtock,
        strip_prefix = "libtock-rs-0f7c97627b7d49dd34129d40717eadba9d307a2d",
        url = "https://github.com/tock/libtock-rs/archive/0f7c97627b7d49dd34129d40717eadba9d307a2d.tar.gz",
        sha256 = "98bcd74c21bc8153fee0551ffc752edf8642fa5b5260802e545fdd89e33332ca",
        additional_files_content = {
            "BUILD": crate_build(
                name = "libtock",
                deps = [
                    "//apis/adc",
                    "//apis/air_quality",
                    "//apis/alarm",
                    "//apis/ambient_light",
                    "//apis/buttons",
                    "//apis/buzzer",
                    "//apis/console",
                    "//apis/gpio",
                    "//apis/leds",
                    "//apis/low_level_debug",
                    "//apis/ninedof",
                    "//apis/proximity",
                    "//apis/sound_pressure",
                    "//apis/temperature",
                    "//panic_handlers/debug_panic",
                    "//platform",
                    "//runtime",
                ],
            ),
            "apis/adc/BUILD": crate_build(
                name = "adc",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/air_quality/BUILD": crate_build(
                name = "air_quality",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/alarm/BUILD": crate_build(
                name = "alarm",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/ambient_light/BUILD": crate_build(
                name = "ambient_light",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/buttons/BUILD": crate_build(
                name = "buttons",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/buzzer/BUILD": crate_build(
                name = "buzzer",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/console/BUILD": crate_build(
                name = "console",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/gpio/BUILD": crate_build(
                name = "gpio",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/leds/BUILD": crate_build(
                name = "leds",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/low_level_debug/BUILD": crate_build(
                name = "low_level_debug",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/ninedof/BUILD": crate_build(
                name = "ninedof",
                crate_name = "libtock_{name}",
                deps = [
                    "//platform",
                    "@crate_index//:libm",
                ],
            ),
            "apis/proximity/BUILD": crate_build(
                name = "proximity",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/sound_pressure/BUILD": crate_build(
                name = "sound_pressure",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "apis/temperature/BUILD": crate_build(
                name = "temperature",
                crate_name = "libtock_{name}",
                deps = ["//platform"],
            ),
            "panic_handlers/debug_panic/BUILD": crate_build(
                name = "debug_panic",
                crate_name = "libtock_{name}",
                deps = [
                    "//apis/console",
                    "//apis/low_level_debug",
                    "//platform",
                    "//runtime",
                ],
            ),
            "panic_handlers/small_panic/BUILD": crate_build(
                name = "small_panic",
                crate_name = "libtock_{name}",
                deps = [
                    "//apis/low_level_debug",
                    "//platform",
                    "//runtime",
                ],
            ),
            "platform/BUILD": crate_build(
                name = "platform",
                crate_name = "libtock_{name}",
            ),
            "runtime/BUILD": crate_build(
                name = "runtime",
                crate_name = "libtock_{name}",
                crate_features = ["no_auto_layout"],
                deps = [
                    "//platform",
                ],
            ),
        },
    )

    # TODO(cfrantz): Currently, elf2tab miscomputes the apps entry point by the
    # size of the TBF header.  My private fork of elf2tab adjusts the start
    # address to the correct entry point.
    http_archive_or_local(
        name = "elf2tab",
        local = "../elf2tab",
        url = "https://github.com/cfrantz/elf2tab/archive/d1bf5e392cae54aa021070319a0eb0488d745efb.tar.gz",
        sha256 = "e2da2e32f81f6d45c827e65092069431e153a6661fe2d07beab3f37fe2e7c2cd",
        strip_prefix = "elf2tab-d1bf5e392cae54aa021070319a0eb0488d745efb",
        build_file = "//third_party/tock:BUILD.elf2tab.bazel",
    )
