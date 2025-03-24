// Conditional version of #![no_std]
#![cfg_attr(target_arch = "riscv32", no_std)]

#[cfg(target_arch = "riscv32")]
use {
    core::arch::global_asm,
    core::panic::PanicInfo,
    core::ptr,
};


pub mod graphics;

/******************************************************************************
 * Reset handling
 ******************************************************************************/

// Init stack pointer
// This has to be before Reset in linker script since otherwise function can't
// enter
#[cfg(target_arch = "riscv32")]
global_asm!("
    .section .text.start
    la sp, _stack_start
");

// Entry point of program
// Sets up abi and initializes memory
#[no_mangle]
#[link_section = ".text.reset"]
#[cfg(target_arch = "riscv32")]
unsafe extern "C" fn Reset() -> ! {

    // Occasionaly useful for debuggin, should write to screen
    // If it doesn't then something is probably wrong with hardware
    // let base_addr = 0x20E000 as *mut u32;
    // let update_addr = 0x20E1FC as *mut u32;
    // base_addr.write_volatile(0b11111111111110010011);
    // update_addr.write_volatile(0b101);

    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    // Copy static variables
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[panic_handler]
#[cfg(target_arch = "riscv32")]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/******************************************************************************
 * Macros
 ******************************************************************************/

/**
 * Macro to define entry point of program in a type safe way.
 *
 * To use, define a divergent function (a function with `-> !`) and call this macro:
 * ```
 * entry!(main);
 *
 * fn main() -> ! {
 *     // Your code here
 *  }
 * ```
 */
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    }
}

