// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENTITAN_SW_DEVICE_SILICON_CREATOR_LIB_BASE_CHIP_H_
#define OPENTITAN_SW_DEVICE_SILICON_CREATOR_LIB_BASE_CHIP_H_

/**
 * @file
 * @brief Chip-level constants.
 */

/**
 * Manifest size for boot stages stored in flash (in bytes).
 */
#define CHIP_MANIFEST_SIZE 1024

/**
 * Manifest format major and minor versions.
 */
#define CHIP_MANIFEST_VERSION_MINOR_1 0x6c47
#define CHIP_MANIFEST_VERSION_MAJOR_1 0x71c3

/**
 * Number of entries in the manifest extensions table.
 */
#define CHIP_MANIFEST_EXT_TABLE_ENTRY_COUNT 15

/**
 * ROM_EXT manifest identifier (ASCII "OTRE").
 */
#define CHIP_ROM_EXT_IDENTIFIER 0x4552544f

/**
 * Allowed bounds for the `length` field of a ROM_EXT manifest.
 */
#define CHIP_ROM_EXT_SIZE_MIN CHIP_MANIFEST_SIZE
#define CHIP_ROM_EXT_SIZE_MAX 0x10000

/**
 * Size of the header of a boot services message.
 */
#define CHIP_BOOT_SVC_MSG_HEADER_SIZE 44

/**
 * Maximum payload size for a boot services message.
 */
#define CHIP_BOOT_SVC_MSG_PAYLOAD_SIZE_MAX 256

/**
 * Maximum size of a boot services message.
 */
// TODO: Has to be a literal because of OT_ASSERT_SIZE. Add an assertion that
// checks if this is equal to header + max_payload.
#define CHIP_BOOT_SVC_MSG_SIZE_MAX 300

/**
 * First owner boot stage, e.g. BL0, manifest identifier (ASCII "OTB0").
 */
#define CHIP_BL0_IDENTIFIER 0x3042544f

/**
 * Allowed bounds for the `length` field of a first owner boot stage manifest.
 */
#define CHIP_BL0_SIZE_MIN CHIP_MANIFEST_SIZE
#define CHIP_BL0_SIZE_MAX 0x70000

/**
 * Value written to the end of the retention SRAM creator area by `test_rom` to
 * be able to determine the type of ROM in tests (ASCII "TEST").
 */
#define TEST_ROM_IDENTIFIER 0x54534554

/**
 * Pinmux pull up/down wait delay.
 *
 * After enabling the pull-up/down on a pin, we need to wait for ~5us for the
 * configuration to propagate to the physical pads. 5us is 500 clock cycles
 * assuming a 100MHz clock.
 */
#define PINMUX_PAD_ATTR_PROP_CYCLES 500

/**
 * Pinmux peripheral input values for software strap pins.
 */
#define SW_STRAP_0_PERIPH 22
#define SW_STRAP_1_PERIPH 23
#define SW_STRAP_2_PERIPH 24

/**
 * Pinmux MIO input selector values for software strap pins.
 */
#define SW_STRAP_0_INSEL 24
#define SW_STRAP_1_INSEL 25
#define SW_STRAP_2_INSEL 26

/**
 * Pads of the software strap pins.
 */
#define SW_STRAP_0_PAD 22
#define SW_STRAP_1_PAD 23
#define SW_STRAP_2_PAD 24

/**
 * Mask for the software strap pins.
 */
#define SW_STRAP_MASK                                    \
  ((1 << SW_STRAP_2_PERIPH) | (1 << SW_STRAP_1_PERIPH) | \
   (1 << SW_STRAP_0_PERIPH))

/**
 * RMA entry strap value.
 *
 * We expect strong pull-ups on SW_STRAP_2_PERIPH and SW_STRAP_1_PERIPH, and
 * strong pull-down on SW_STRAP_0_PERIPH, i.e. `11_11_00`.
 */
#define SW_STRAP_RMA_ENTRY                               \
  ((1 << SW_STRAP_2_PERIPH) | (1 << SW_STRAP_1_PERIPH) | \
   (0 << SW_STRAP_0_PERIPH))

/**
 * Bootstrap strap value.
 *
 * We expect strong pull-ups on all software strap pins, i.e. `11_11_11`.
 */
#define SW_STRAP_BOOTSTRAP                               \
  ((1 << SW_STRAP_2_PERIPH) | (1 << SW_STRAP_1_PERIPH) | \
   (1 << SW_STRAP_0_PERIPH))

#endif  // OPENTITAN_SW_DEVICE_SILICON_CREATOR_LIB_BASE_CHIP_H_
