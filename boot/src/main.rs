// SPDX-License-Identifier: GPL-3.0
// Copyright (C) 2026 noahbelton29

#![no_main]
#![no_std]

use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};
use uefi::{boot, prelude::*, system};

const BYTES_PER_PIXEL: usize = 4;

#[entry]
fn main() -> Status {
    if let Err(err) = draw_rect() {
        return err.status();
    }
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

fn draw_rect() -> uefi::Result {
    system::with_stdout(|console| console.clear())?;

    // open graphics device
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    // read screen properties
    let mode = gop.current_mode_info();
    let (screen_w, screen_h) = mode.resolution();

    // rows can be padded in memory, so always step rows by stride and not width
    let stride = mode.stride();

    let (red, green, blue) = (0x20, 0x90, 0xFF);
    let color: [u8; BYTES_PER_PIXEL] = match mode.pixel_format() {
        PixelFormat::Rgb => [red, green, blue, 0],
        PixelFormat::Bgr => [blue, green, red, 0],
        // bitmask needs its own custom handling, BltOnly has no framebuffer
        _ => return Err(Status::UNSUPPORTED.into()),
    };

    // find framebuffer
    let framebuffer = gop.frame_buffer().as_mut_ptr();

    // rect properties
    let (rect_w, rect_h) = (400, 200);
    let top = (screen_h - rect_h) / 2;
    let bottom = top + rect_h;
    let left = (screen_w - rect_w) / 2;
    let right = left + rect_w;

    // draw the rect
    for y in top..bottom {
        for x in left..right {
            let offset = (y * stride + x) * BYTES_PER_PIXEL;

            unsafe  {
                framebuffer
                    .add(offset)
                    .cast::<[u8; BYTES_PER_PIXEL]>()
                    .write_volatile(color);
            }
        }
    }


    Ok(())
}