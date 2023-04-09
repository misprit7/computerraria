#![no_std]
#![no_main]

mod maps;
mod raycaster;

use maps::MAP;
use tdriver::entry;
use tdriver::graphics;
use tdriver::graphics::Screen;

entry!(main);

fn main() -> ! {
    let mut screen = graphics::init();
    let raycaster = raycaster::Raycaster::new(MAP, 60.0);
    let mut pixels: [[bool; graphics::WIDTH]; graphics::HEIGHT] =
        [[false; graphics::WIDTH]; graphics::HEIGHT];

    let mut angle = 0.0;
    loop {
        raycaster.render(2.5, 3.5, angle, &mut pixels);
        angle += 0.04;
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
