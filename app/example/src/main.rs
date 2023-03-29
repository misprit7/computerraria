#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;

entry!(main);

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;
// static mut A: u8 = 1;
// static mut B: u8 = 1;

// Entry point of user code
fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };
    // graphics::sanity_check();
    loop {
        // let mut arr: [[u32; 2]; 48] = [[1; 2]; 48];
        let mut arr: [u64; 48] = [1; 48];
        graphics::write_raw(&arr);
        graphics::update();
    }
}
