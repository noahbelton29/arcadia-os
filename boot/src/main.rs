// SPDX-License-Identifier: GPL-3.0
// Copyright (C) 2026 noahbelton29

#![no_main]
#![no_std]

use uefi::{cstr16, prelude::*, system};

#[entry]
fn main() -> Status {
    let _ = system::with_stdout(|out| {
        out.clear()?;
        out.output_string(cstr16!("Hello world!\r\n"))
    });

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}