#![allow(clippy::cast_precision_loss)]

use crate::{darksiders1::gfc, library::fonts::ibm_vga_8x14};
use na::Vector4;
use std::convert::TryFrom;

pub fn draw_string(renderer: &gfc::UIRenderer, x: f32, y: f32, scale: i32, s: &str) {
    let mut dx = 0;
    let mut dy = 0;
    for ch in s.chars() {
        if ch == '\n' {
            dx = 0;
            dy += 1;
            continue;
        }
        draw_char(
            renderer,
            x + scale as f32 * (dx * ibm_vga_8x14::WIDTH) as f32,
            y + scale as f32 * (dy * ibm_vga_8x14::HEIGHT) as f32,
            scale,
            ch,
        );
        dx += 1;
    }
}

fn draw_char(renderer: &gfc::UIRenderer, x: f32, y: f32, scale: i32, ch: char) {
    let ch = if (ch as usize) < 256 {
        ch as usize
    } else {
        1 // :)
    };
    for row in 0..ibm_vga_8x14::HEIGHT {
        for col in 0..8 {
            let bits = ibm_vga_8x14::DATA[ch * ibm_vga_8x14::HEIGHT + row];
            let on = bits & (1 << (7 - col)) != 0;
            if on {
                renderer.fill_rect(
                    x + (col * usize::try_from(scale).unwrap()) as f32,
                    y + (row * usize::try_from(scale).unwrap()) as f32,
                    scale as f32,
                    scale as f32,
                    &Vector4::new(0.0, 0.0, 1.0, 1.0),
                    &Vector4::new(0.0, 0.0, 1.0, 1.0),
                );
            }
        }
    }
}
