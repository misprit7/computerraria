#[cfg(not(target_arch = "riscv32"))]
use {
    tui::{
        backend::{Backend, CrosstermBackend},
        layout::{Constraint, Direction, Layout},
        widgets::{
            Block, Borders,
            canvas::{Canvas, Rectangle},
        },
        style::Color,
        Terminal, symbols, Frame,
    },
    std::{io, time::{Duration, Instant}, process},
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

/**
 * Struct describing the current state of screen
 */
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
    tick_rate: Duration,
    #[cfg(not(target_arch = "riscv32"))]
    last_tick: Instant,
}


/******************************************************************************
 * x86 HAL
 ******************************************************************************/

#[cfg(not(target_arch = "riscv32"))]
fn write_screen(screen: &mut Screen) {
    while screen.last_tick.elapsed() < screen.tick_rate {
        screen.terminal.draw(|f| { ui(f, &screen.state) }).unwrap();
            if event::poll(Duration::from_millis(0)).unwrap() {
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
                        process::exit(0)
                    }, 
                    _ => {}
                }
            }
        }
    }

    screen.last_tick = Instant::now();
}

#[cfg(not(target_arch = "riscv32"))]
fn ui<B: Backend>(f: &mut Frame<B>, state: &[[bool; WIDTH]; HEIGHT]) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((HEIGHT) as u16), Constraint::Min(0)
            ].as_ref()
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(((WIDTH+1) * 2) as u16), Constraint::Min(0)
            ].as_ref()
        )
        .split(vertical_chunks[0]);

    let canvas = Canvas::default()
        .marker(symbols::Marker::Braille)
        .block(Block::default().borders(Borders::ALL).title("Screen"))
        .paint(|ctx| {
            for h in 0..state.len() {
                for w in 0..state[0].len() {
                    if state[h][w] {
                        ctx.draw(&Rectangle {
                            x: w as f64, y: HEIGHT as f64 - h as f64-1.0,
                            width: 1.0, height: 1.0,
                            color: Color::White
                        });
                    }
                }
            }
        })
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64]);
    f.render_widget(canvas, chunks[0]);
}

/******************************************************************************
 * Public Functions
 ******************************************************************************/

/**
 * Initializes screen and returns screen struct
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
        tick_rate: Duration::from_millis(100),
        last_tick: Instant::now()
    };

    write_screen(&mut screen);

    screen
}}

/**
 * Writes a pixel array into screen as implemented in hardware
 */
pub fn write_raw(screen: &mut Screen, pixels: &[[u32; WORDS]; HEIGHT]) {

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
    for h in 0..screen.state.len() {
        for w in 0..screen.state[0].len() {
            screen.state[h][w] = (pixels[w/32][h] >> (w%32)) & 0b1 == 1
        }
    }
}}


/**
 * Equivalent to write_raw except with u64 instead of [u32; 2]
 */
pub fn write_long(screen: &mut Screen, pixels: &[u64; HEIGHT]) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        for h in 0..pixels.len(){
            screen.base_addr.add(2 * h).write_volatile(pixels[h] as u32);
            screen.base_addr.add(2 * h + 1).write_volatile((pixels[h] >> 32) as u32);
        }
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    for h in 0..screen.state.len() {
        for w in 0..screen.state[0].len() {
            screen.state[h][w] = (pixels[h] >> w) & 0b1 == 1
        }
    }
}}

/**
 * Writes a line to screen at a specified row, word is 0 for lower word and 1 for upper
 */
pub fn write_line(screen: &mut Screen, data: u32, row: usize, word: usize) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        screen.base_addr.add((2*row + word) as usize).write_volatile(data);
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    for w in 0..32 {
        screen.state[row as usize][(word * 32 + w) as usize] = (data >> w) & 0b1 == 1;
    }
}}

/**
 * Updates screen by writing to screen register
 */
pub fn update(screen: &mut Screen) {

#[cfg(target_arch = "riscv32")] {
    unsafe {
        screen.update_addr.write_volatile(0b1);
    }
}

#[cfg(not(target_arch = "riscv32"))] {
    write_screen(screen);
}}

/******************************************************************************
 * Legacy Functions
 ******************************************************************************/

/**
 * Sanity check to ensure all pixels in screen work by running a pixel accross the entire screen
 *
 * This is kept as super low level/hardcoded intentionally to prevent other stuff in this module
 * from breaking this, might change this later when I'm more confident in my code
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

