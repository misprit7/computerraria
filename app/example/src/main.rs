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
    let screen: *mut u32 = 0x1E000 as *mut u32;
    let screen_write: *mut u32 = 0x1E1FC as *mut u32;
    graphics::tester();
    unsafe {
        // let (mut a, mut b) = (1, 1);
        loop {
            // (b, a) = (a, a+b);
            for i in 0..96 {
                for j in 0..32 {
                    screen.add(i).write_volatile(1 << j);
                    screen_write.write_volatile(1);
                }
                screen.add(i).write_volatile(0);
            }
        }
    }
}
