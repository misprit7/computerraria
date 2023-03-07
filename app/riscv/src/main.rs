#![no_std]
#![no_main]

// #![allow(stable_features)]
// #![feature(core_intrinsics)]

use core::panic::PanicInfo;
// use core::intrinsics;
// use core::ptr;

// fn main() {
    // unsafe {
    //     let ptr: *mut u8 = 0b10000 as *mut u8;
    //     *ptr = 2
    // }

// }

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;
static mut A: u8 = 1;
static mut B: u8 = 1;

#[no_mangle]
fn main() {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };
    unsafe {
        // let (mut a, mut b) = (1, 1);
        let ptr: *mut u8 = 0b10000 as *mut u8;

        loop {
            (B, A) = (A, A+B);
            *ptr = B;
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

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

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // unsafe { intrinsics::abort() }
    loop {}
}
