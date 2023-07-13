#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;

mod video;

entry!(main);

// Entry point of user code
fn main() -> ! {
    let mut screen = graphics::init();
    let mut i = 0;
    loop {
        if i >= video::FRAMES.len() {
            i = 0;
        }
        graphics::write_raw(&mut screen, &video::FRAMES[i]);
        i += 1;
        graphics::update(&mut screen);
    }
}
