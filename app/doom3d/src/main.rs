// cargo run --target x86_64-pc-windows-msvc	

// #![no_std]
#![no_main]

mod raycaster;
mod maps;

use core::cmp::min;
use core::ops::{Add, Mul};
use tdriver::entry;
use tdriver::graphics;
use tdriver::graphics::Screen;
use tdriver::graphics::HEIGHT;
use tdriver::graphics::WIDTH;
use raycaster::*;
use maps::*;

// const MAP: [[bool; raycaster::MAP_WIDTH]; raycaster::MAP_HEIGHT] =
//     [[true, true, true, true, true], 
//     [true, false, false, false, true],
//     [true, false, false, false, true],
//     [true, false, false, false, true],
//     [true, true, true, true, true]];

entry!(main);

fn main() -> ! {
    let mut screen = graphics::init();
    let raycaster = raycaster::Raycaster::new(BIG_MAP, 70.0);
    let mut pixels: [[bool; graphics::WIDTH]; graphics::HEIGHT] = [[false; graphics::WIDTH]; graphics::HEIGHT];

    // raycaster.render(1.5, 2.5, 0.0, &mut pixels);

    let mut angle = 0.0;

    loop {
        raycaster.render(5.0, 5.0, angle, &mut pixels);
        angle += 0.05;
        update_screen(&mut screen, &pixels);
    }
}

fn update_screen(screen: &mut Screen, pixels: &[[bool; graphics::WIDTH]; graphics::HEIGHT]) {
    for y_pix in 0..graphics::HEIGHT {
        let mut word: u64 = 0;
        for x_pix in 0..graphics::WIDTH {
            if pixels[y_pix][x_pix] {
                word |= 0b1 << x_pix;
            }
        }

        graphics::write_line(screen, word as u32, y_pix, 0);
        graphics::write_line(screen, (word >> 32) as u32, y_pix, 1);
    }

    graphics::update(screen);
}
