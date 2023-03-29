
/******************************************************************************
 * Constants
 ******************************************************************************/

const SCREEN_BASE: *mut u32 = 0x1E000 as *mut u32;
const SCREEN_UPDATE: *mut u32 = 0x1E1FC as *mut u32;

/// Number of pixels in horizontal direction
pub const WIDTH: usize = 64;
/// Number of pixels in vertical direction
pub const HEIGHT: usize = 48;

/// Width in number of 32 bit words that can fit horizontally
pub const WORDS: usize = WIDTH / 32;

/******************************************************************************
 * Public Functions
 ******************************************************************************/

/**
 * Writes a pixel array into screen as implemented in hardware
 */
pub fn write_raw(pixels: &[[u32; WORDS]; HEIGHT]) {
    unsafe {
        for w in 0..WORDS {
            for h in 0..HEIGHT{
                SCREEN_BASE.add(w + WORDS * h).write_volatile(pixels[h][w]);
            }
        }
    }
}


/**
 * Equivalent to write_raw except with u64 instead of [u32; 2]
 */
pub fn write_long(pixels: &[u64; HEIGHT]) {
    unsafe {
        for h in 0..HEIGHT{
            SCREEN_BASE.add(2 * h).write_volatile(pixels[h] as u32);
            SCREEN_BASE.add(2 * h + 1).write_volatile((pixels[h] >> 32) as u32);
        }
    }
}

/**
 * Updates screen by writing to screen register
 */
pub fn update() {
    unsafe {
        SCREEN_UPDATE.write_volatile(0b1);
    }
}

/******************************************************************************
 * Legacy Functions
 ******************************************************************************/

/**
 * Sanity check to ensure all pixels in screen work by running a pixel accross the entire screen
 *
 * This is kept as super low level/hardcoded intentionally to prevent other stuff in this module
 * from breaking this, might change this later when I'm more confident in my code
 */
pub fn sanity_check() {
    let screen: *mut u32 = 0x1E000 as *mut u32;
    let screen_write: *mut u32 = 0x1E1FC as *mut u32;
    unsafe {
        for i in 0..96 {
            for j in 0..32 {
                screen.add(i).write_volatile(1 << j);
                screen_write.write_volatile(1);
            }
            screen.add(i).write_volatile(0);
        }
    }
}

