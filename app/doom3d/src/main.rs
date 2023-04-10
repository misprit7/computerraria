#![no_std]
#![no_main]

mod maps;
mod raycaster;

use maps::MAP;
use tdriver::entry;
use tdriver::graphics;
use tdriver::graphics::Screen;

use fixed::types::{I4F12, I9F7};

entry!(main);

fn main() -> ! {
    let mut screen = graphics::init();
    let raycaster = raycaster::Raycaster::new(MAP, I9F7::from_num(60.0));
    let mut pixels: [[bool; graphics::WIDTH]; graphics::HEIGHT] =
        [[false; graphics::WIDTH]; graphics::HEIGHT];

    let mut angle = I4F12::const_from_int(0);
    loop {
        raycaster.render(I4F12::from_num(2.5), I4F12::from_num(3.5), angle, &mut pixels);
        angle += I4F12::from_num(0.04);
        angle %= 2 * I4F12::PI;
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
