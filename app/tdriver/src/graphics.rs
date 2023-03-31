#[cfg(not(target_arch = "riscv32"))]
use {
    tui::{
        backend::CrosstermBackend, 
        widgets::{
            Block, Borders,
            canvas::{Canvas},
        },
        Terminal,
    },
    std::{io, thread, time::Duration},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
};

/******************************************************************************
 * Constants
 ******************************************************************************/

/// Number of pixels in horizontal direction
pub const WIDTH: usize = 64;
/// Number of pixels in vertical direction
pub const HEIGHT: usize = 48;

/// Width in number of 32 bit words that can fit horizontally
pub const WORDS: usize = WIDTH / 32;

/******************************************************************************
 * Types
 ******************************************************************************/

pub struct Screen {
    #[cfg(target_arch = "riscv32")]
    base_addr: *mut u32,
    #[cfg(target_arch = "riscv32")]
    update_addr: *mut u32,

    #[cfg(not(target_arch = "riscv32"))]
    terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
    #[cfg(not(target_arch = "riscv32"))]
    state: [[bool; WIDTH]; HEIGHT],
    #[cfg(not(target_arch = "riscv32"))]
    timeout: Duration
}

/******************************************************************************
 * x86 HAL
 ******************************************************************************/

#[cfg(not(target_arch = "riscv32"))]
fn write_screen(screen: &mut Screen) {
    screen.terminal.draw(|f| {
        let size = f.size();
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Screen"))
            .paint(|_| {})
            .x_bounds([0.0, WIDTH as f64])
            .y_bounds([0.0, HEIGHT as f64]);
        f.render_widget(canvas, size);
    }).unwrap();
    if event::poll(screen.timeout).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code { 
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    execute!(
                        screen.terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    ).unwrap();
                    screen.terminal.show_cursor().unwrap();
                    return
                }, 
                _ => {}
            }
        }
    }
}

/******************************************************************************
 * Public Functions
 ******************************************************************************/

/**
 * Initializes screen and returns screeiguration struct
 */
pub fn init() -> Screen {

#[cfg(target_arch = "riscv32")] {
    Screen {
        base_addr: 0x1E000 as *mut u32,
        update_addr: 0x1E1FC as *mut u32,
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    let mut screen = Screen { 
        terminal,
        state: [[false; WIDTH]; HEIGHT],
        timeout: Duration::from_millis(100)
    };

    write_screen(&mut screen);

    screen
}}

/**
 * Writes a pixel array into screen as implemented in hardware
 */
pub fn write_raw(screen: &Screen, pixels: &[[u32; WORDS]; HEIGHT]) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        for w in 0..WORDS {
            for h in 0..HEIGHT{
                screen.base_addr.add(w + WORDS * h).write_volatile(pixels[h][w]);
            }
        }
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    // println!("test");
}}


/**
 * Equivalent to write_raw except with u64 instead of [u32; 2]
 */
pub fn write_long(screen: &Screen, pixels: &[u64; HEIGHT]) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        for h in 0..pixels.len(){
            screen.base_addr.add(2 * h).write_volatile(pixels[h] as u32);
            screen.base_addr.add(2 * h + 1).write_volatile((pixels[h] >> 32) as u32);
        }
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    // println!("test");
}}

/**
 * Updates screen by writing to screen register
 */
pub fn update(screen: &Screen) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        screen.update_addr.write_volatile(0b1);
    }
}

#[cfg(not(target_arch = "riscv32"))] {

}}

/******************************************************************************
 * Legacy Functions
 ******************************************************************************/

/**
 * Sanity check to ensure all pixels in screen work by running a pixel accross the entire screen
 *
 * This is kept as super low level/hardcoded intentionally to prevent other stuff in this module
 * from breaking this, might change this later when I'm more screeident in my code
 */
#[cfg(target_arch = "riscv32")]
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

