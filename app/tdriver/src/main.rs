#![no_std]
#![no_main]

// #![allow(stable_features)]
// #![feature(core_intrinsics)]

use core::panic::PanicInfo;
use core::arch::asm;
// use core::ptr;

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;
static mut A: u8 = 1;
static mut B: u8 = 1;

// Entry point of user code
#[no_mangle]
fn main() {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };
    let screen: *mut u32 = 0x1E000 as *mut u32;
    let screen_write: *mut u32 = 0x1E1FC as *mut u32;
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


// Entry point of program
// Sets up abi and initializes memory
#[no_mangle]
#[link_section = ".text.start"]
pub unsafe extern "C" fn Reset() -> ! {

    // Init stack pointer
    asm!(
        "la sp, _stack_start"
    );

    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    // custom memcpy implementation for size
    // These are not good for alignment, TODO: improve these or replace
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    // ptr::write_bytes(&mut _sbss as *mut u8, 0, count);
    let mut p = &mut _sbss as *mut u8;
    for _ in 0..count {
        *p = 0;
        p = p.add(1);
    }

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    // ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
    let mut p1 = &_sidata as *const u8;
    let mut p2 = &mut _sdata as *mut u8;
    for _ in 0..count {
        *p2 = *p1;
        p1 = p1.add(1);
        p2 = p2.add(1);
    }

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
