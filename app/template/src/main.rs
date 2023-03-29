#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;

entry!(main);

// Entry point of user code
fn main() -> ! {
    loop {
        let arr: [u64; graphics::HEIGHT] = [0xF00000000000000F; graphics::HEIGHT];
        graphics::write_long(&arr);
        graphics::update();
    }
}
