// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_main]
#![no_std]
use core::fmt::Write;
use libtock::console::Console;
use libtock::runtime::{set_main, stack_size};

set_main!(main);
stack_size!(0x800);

use core::cell::Cell;
use libtock_platform as platform;
use libtock_platform::allow_ro::AllowRo;
use libtock_platform::allow_rw::AllowRw;
use libtock_platform::share;
use libtock_platform::subscribe::Subscribe;
use libtock_platform::{DefaultConfig, ErrorCode, Syscalls};


pub struct Hmac<S: Syscalls, C: Config = DefaultConfig>(S, C);

/// System call configuration trait for `Hmac`.
pub trait Config:
    platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config
{
}
impl<T: platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config>
    Config for T
{
}


impl<S: Syscalls, C: Config> Hmac<S, C> {
    // /// Run a check against the HMAC capsule to ensure it is present.
    // ///
    // /// Returns `true` if the driver was present. This does not necessarily mean
    // /// that the driver is working, as it may still fail to allocate grant
    // /// memory.
    // #[inline(always)]
    // pub fn driver_check() -> bool {
    //     S::command(DRIVER_NUM, command::DRIVER_CHECK, 0, 0).is_success()
    // }

    pub fn do_hmac(key: &[u8], data: &[u8], dest: &mut [u8]) -> Result<(), ErrorCode> {
        // share::scope::<
        //     (
        //     ),
        //     _,
        //     _,
        // >(|_| {

	//     // Set HMAC-SHA256 algorithm:
	//     S::command(DRIVER_NUM, command::SET_ALGORITHM, 0, 0).to_result()?;

	//     // Run:
        //     // S::command(DRIVER_NUM, command::RUN, 0, 0).to_result()?;

        //     loop {
        //         // S::yield_wait();
	// 	panic!("Done!");
        //         return Ok(());
        //     }
        // })

	// Set HMAC-SHA256 algorithm:
	S::command(DRIVER_NUM, command::SET_ALGORITHM, 0, 0).to_result()?;
	// S::command(DRIVER_NUM, command::SET_ALGORITHM, 0, 0).to_result()?;

	let called: Cell<Option<(u32, u32)>> = Cell::new(None);
        share::scope::<
            (
                AllowRo<_, DRIVER_NUM, { allow_ro::KEY }>,
		AllowRo<_, DRIVER_NUM, { allow_ro::DATA }>,
		AllowRw<_, DRIVER_NUM, { allow_rw::DEST }>,
                Subscribe<_, DRIVER_NUM, { subscribe::DONE }>,
            ),
            _,
            _,
        >(|handle| {
            let (allow_key, allow_data, allow_dest, subscribe_done) = handle.split();

            S::allow_ro::<C, DRIVER_NUM, { allow_ro::KEY }>(allow_key, key)?;
	    S::allow_ro::<C, DRIVER_NUM, { allow_ro::DATA }>(allow_data, data)?;
	    S::allow_rw::<C, DRIVER_NUM, { allow_rw::DEST }>(allow_dest, dest)?;

            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::DONE }>(subscribe_done, &called)?;

	    // Run:
            S::command(DRIVER_NUM, command::RUN, 0, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((res, val)) = called.get() {
		// panic!("Done!");
		panic!("Done: {:?} {:?}", res, val);
                    return Ok(());
                }
            }
        })
    }

}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x40003;

// Command IDs
#[allow(unused)]
mod command {
    pub const SET_ALGORITHM: u32 = 0;
    pub const RUN: u32 = 1;
    pub const UPDATE: u32 = 2;
    pub const FINISH: u32 = 3;
    pub const VERIFY: u32 = 4;
    pub const VERIFY_FINISH: u32 = 5;
}

#[allow(unused)]
mod subscribe {
    pub const DONE: u32 = 0;
}

#[allow(unused)]
mod allow_ro {
    pub const KEY: u32 = 0;
    pub const DATA: u32 = 1;
    pub const COMPARE: u32 = 2;
}

#[allow(unused)]
mod allow_rw {
    pub const DEST: u32 = 0;
}


fn main() {
    write!(Console::writer(), "Hello OpenTitan!\r\n").unwrap();

    let data = b"Hello World!";
    let key = [0; 32];
    let mut out = [0; 32];

    Hmac::<libtock_runtime::TockSyscalls, libtock_platform::DefaultConfig>::do_hmac(&key[..], &data[..], &mut out[..]).unwrap();

    write!(Console::writer(), "Done! {:x?}\r\n", out).unwrap();
}
