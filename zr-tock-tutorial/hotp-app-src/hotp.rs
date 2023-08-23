// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_main]
#![no_std]
use core::fmt::Write;
use core::num::NonZeroUsize;
use libtock::console::Console;
use libtock::platform::ErrorCode;
use libtock::runtime::{set_main, stack_size};

mod hmac;

set_main!(main);
stack_size!(0x800);

const MAX_KEY_BYTES: usize = 64;
const KEY_SLOTS: usize = 4;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum KeyDigits {
    Six = 6,
    Seven = 7,
    Eight = 8,
}

struct KeyState {
    /// Key length in bytes
    len: NonZeroUsize,
    /// Encrypted key (to be decrypted with the encryption oracle)
    enc_key: [u8; MAX_KEY_BYTES],
    /// AES128-CTR encryption initialization vector
    iv: [u8; 16],
    /// Digits of this HOTP key (determines output encoding)
    digits: KeyDigits,
    /// Moving factor (counter)
    moving_factor: u64,
}

// Bytewise read until hitting end of buffer or a newline character.
//
// Returns the number of bytes read, and whether the function returned because
// of a buffer overrun or hitting a newline char.
fn readline(buf: &mut [u8]) -> Result<(usize, bool), ErrorCode> {
    let mut idx = 0;

    while { idx < buf.len() && (idx == 0 || buf[idx - 1] != b'\n') } {
        let (count, res) = Console::read(&mut buf[idx..idx + 1]);
        let () = res?;

        if count > 0 && buf[idx + count - 1] == b'\r' {
            buf[idx + count - 1] = b'\n';
        }

        if count == 1 && buf[idx] == b'\n' {
            // Echo back nothing
        } else if count > 1 && buf[idx] == b'\n' {
            // Echo back everything except for the last byte
            Console::write(&buf[idx..(idx + count - 1)])?;
        } else {
            // Echo back everything
            Console::write(&buf[idx..(idx + count)])?;
        }

        idx += count;
    }

    // Print a newline at the end:
    Console::write(b"\r\n")?;

    Ok((
        idx,
        idx >= buf.len() && buf.last().map_or(true, |c| *c != b'\n'),
    ))
}

// TODO: implement calls to encryption oracle
fn oracle_crypt(_iv: &[u8; 16], src: &[u8], dst: &mut [u8]) -> Result<(), ErrorCode> {
    dst.copy_from_slice(src);
    Ok(())
}

fn gen_token(key: &KeyState) -> Result<bool, ErrorCode> {
    let key_len = key.len.get();
    let mut dec_key: [u8; MAX_KEY_BYTES] = [0; MAX_KEY_BYTES];

    // Decrypt the stored key:
    oracle_crypt(&key.iv, &key.enc_key[..key_len], &mut dec_key[..key_len])?;

    // Generate the data to perform the HMAC operation over: in the case of
    // HOTP/TOTP this is the moving factor in big endian:
    let hmac_data = u64::to_be_bytes(key.moving_factor);

    // Perform the HMAC operation. The output buffer's size will vary depending
    // on the underlying hash algorithm. For now we only support SHA-256 with a
    // 32-bit output:
    let mut hmac_out: [u8; 32] = [0; 32];
    hmac::Hmac::<libtock::runtime::TockSyscalls, libtock::platform::DefaultConfig>::do_hmac_sha256(
        &dec_key[..key_len],
        &hmac_data,
        &mut hmac_out[..],
    )?;

    // We construct the HOTP value based on four bytes of the output buffer,
    // extracted from the HMAC output at a given offset, which in turn is also
    // determined by the HMAC output:
    let offset: usize = (hmac_out.last().copied().unwrap() & 0x0F) as usize;
    let mut tok: u32 = u32::from_be_bytes([
        hmac_out[offset] & 0x7f,
        hmac_out[offset + 1],
        hmac_out[offset + 2],
        hmac_out[offset + 3],
    ]);

    // Limit output to requested number of digits. Modulus by 10^digits:
    let digit_mod = 10_u32.pow(key.digits as u8 as u32);
    tok %= digit_mod;

    // Finally, print the generated token:
    write!(
        Console::writer(),
        "Token (moving factor = {factor}): {tok:0>digits$}\r\n",
        // "Token (moving factor = {factor}): {tok}\r\n",
        factor = key.moving_factor,
        tok = tok,
        digits = key.digits as u8 as usize
    )
    .map_err(|_| ErrorCode::Fail)?;

    Ok(true)
}

/// Main loop which performs user-interactions and implements HOTP token update
/// & code generation requests. Returns the slot which had its state updated, if
/// any, such that the caller can update any persistent storage accordingly.
fn main_loop(key_store: &mut [Option<KeyState>]) -> Result<Option<usize>, ErrorCode> {
    write!(Console::writer(), "hotp> ").map_err(|_| ErrorCode::Fail)?;

    // Allocate an input buffer of at least twice the maximum key size, such
    // that we can read the Base32-encoded key in one operation, plus a newline
    // (CRLF) character:
    let mut input: [u8; 128 + 2] = [0; 128 + 2];
    let (read_bytes, overrun) = readline(&mut input)?;

    // Parse the slot index from the input string, starting at a given offset
    // and excluding the final newline char:
    fn get_slot(input: &[u8], start_offset: usize, len: usize) -> Option<usize> {
        input
            .get(start_offset..(len - 1))
            .and_then(|sliced| core::str::from_utf8(sliced).ok())
            .and_then(|slot_str| <usize as core::str::FromStr>::from_str(slot_str).ok())
    }

    match (overrun, &input[..read_bytes]) {
        (false, s) if s == &b"help\n"[..] => {
            write!(
                Console::writer(),
                "Available commands:\r\n\
		 -   help            Print this message.\r\n\
		 -   gen <slot>      Generate an OTP token.\r\n\
                 -   reset <slot>    Reset the counter of an OTP slot.\r\n\
                 -   program <slot>  Program an OTP token slot\r\n\
		 OTP token slots available: {}\r\n",
                key_store.len(),
            )
            .map_err(|_| ErrorCode::Fail)?;
            Ok(None)
        }
        (false, s) if s.starts_with(b"gen ") => {
            // let slot_idx_res: Result<usize, ()> = core::str::from_utf8(&s[4..(read_bytes - 1)])
            // 	.map_err(|_| ())
            // 	.and_then(|slot_str| <usize as core::str::FromStr>::from_str(slot_str).map_err(|_| ()));

            if let Some(slot_idx) = get_slot(s, 4, read_bytes) {
                match key_store.get_mut(slot_idx) {
                    None => {
                        write!(
                            Console::writer(),
                            "Error: slot {} not present.\r\n",
                            slot_idx
                        )
                        .map_err(|_| ErrorCode::Fail)?;
                        Ok(None)
                    }
                    Some(None) => {
                        write!(
                            Console::writer(),
                            "Error: slot {} not configured.\r\n",
                            slot_idx
                        )
                        .map_err(|_| ErrorCode::Fail)?;
                        Ok(None)
                    }
                    Some(Some(ref mut key)) => {
                        if gen_token(key)? {
                            key.moving_factor = key.moving_factor.wrapping_add(1);
                            Ok(Some(slot_idx))
                        } else {
                            Ok(None)
                        }
                    }
                }
            } else {
                write!(
                    Console::writer(),
                    "Error: cannot parse OTP token slot index.\r\n"
                )
                .map_err(|_| ErrorCode::Fail)?;
                Ok(None)
            }
        }
        (false, s) if s.starts_with(b"reset ") => {
            if let Some(slot_idx) = get_slot(s, 6, read_bytes) {
                match key_store.get_mut(slot_idx) {
                    None => {
                        write!(
                            Console::writer(),
                            "Error: slot {} not present.\r\n",
                            slot_idx
                        )
                        .map_err(|_| ErrorCode::Fail)?;
                        Ok(None)
                    }
                    Some(None) => {
                        write!(
                            Console::writer(),
                            "Error: slot {} not configured.\r\n",
                            slot_idx
                        )
                        .map_err(|_| ErrorCode::Fail)?;
                        Ok(None)
                    }
                    Some(Some(ref mut key)) => {
                        key.moving_factor = 0;
                        Ok(Some(slot_idx))
                    }
                }
            } else {
                write!(
                    Console::writer(),
                    "Error: cannot parse OTP token slot index.\r\n"
                )
                .map_err(|_| ErrorCode::Fail)?;
                Ok(None)
            }
        }
        (true, _) => {
            write!(
                Console::writer(),
                "Error: input buffer overrun, read aborted.\r\n"
            )
            .map_err(|_| ErrorCode::Fail)?;
            Ok(None)
        }
        (false, _) => {
            write!(
                Console::writer(),
                "Error: unknown command! Type \"help\" to see available commands.\r\n"
            )
            .map_err(|_| ErrorCode::Fail)?;
            Ok(None)
        }
    }
}

fn main() {
    write!(Console::writer(), "Hello, HOTP!\r\n").unwrap();

    let mut key_store: [Option<KeyState>; KEY_SLOTS] = Default::default();

    // Program a dummy key:
    key_store[0] = Some(KeyState {
        len: NonZeroUsize::new(32).unwrap(),
        enc_key: [b'a'; MAX_KEY_BYTES],
        iv: [0; 16],
        digits: KeyDigits::Six,
        moving_factor: 0,
    });

    loop {
        // No persistent storage implemented, so keep invoking the
        // main_loop.
        main_loop(&mut key_store).unwrap();
    }
}
