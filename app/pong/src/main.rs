#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;
use tdriver::graphics::WIDTH;
use tdriver::graphics::HEIGHT;

entry!(main);


// Entry point of user code
fn main() -> ! {
    let mut pixels: [u64; graphics::HEIGHT] = [0x0000000000000000; graphics::HEIGHT];
    let mut ball_pos: (usize, usize) = ((WIDTH / 2) as usize, (HEIGHT / 2) as usize);
    let mut ball_vel: (i8, i8) = (3, 1);
    let mut screen = graphics::init();

    let mut p_left = HEIGHT / 2;
    let mut p_right = HEIGHT / 2;
    let p_size: usize = 5;

    for i in p_left-p_size..=p_left+p_size {
        let data: u32 = 0x00000001;
        graphics::write_line(&mut screen, data, i, 0);
        pixels[i] |= (0xFFFFFFFF & data) as u64;
    }
    for i in p_right-p_size..=p_right+p_size {
        let data: u32 = 0x80000000;
        graphics::write_line(&mut screen, data, i, 1);
        pixels[i] |= (data as u64) << 32;
    }
    graphics::update(&mut screen);

    loop {

        // Move left platform
        let defense_left = ball_vel.0 < 0;
        if (defense_left && ball_pos.1 > p_left) || (!defense_left && p_left < HEIGHT / 2) {
            if p_left + p_size < HEIGHT - 1 {
                graphics::write_line(&mut screen, 0, p_left-p_size, 0);
                pixels[p_left-p_size] &= !1;
                graphics::write_line(&mut screen, 1, p_left+p_size, 0);
                pixels[p_left+p_size] |= 1;
                p_left += 1;
            }
        } else {
            if p_left - p_size > 0 {
                graphics::write_line(&mut screen, 1, p_left-p_size, 0);
                pixels[p_left-p_size] |= 1;
                graphics::write_line(&mut screen, 0, p_left+p_size, 0);
                pixels[p_left+p_size] &= !1;
                p_left -= 1;
            }
        }

        // Move right platform
        let defense_right = ball_vel.0 > 0;
        if (defense_right && ball_pos.1 > p_right) || (!defense_right && p_right < HEIGHT / 2) {
            if p_right + p_size < HEIGHT - 1 {
                graphics::write_line(&mut screen, 0, p_right-p_size, 1);
                pixels[p_right-p_size] &= !(1 << 63);
                graphics::write_line(&mut screen, 1 << 31, p_right+p_size, 1);
                pixels[p_right+p_size] |= 1 << 63;
                p_right += 1;
            }
        } else {
            if p_right - p_size > 0 {
                graphics::write_line(&mut screen, 1 << 31, p_right-p_size, 1);
                pixels[p_right-p_size] |= 1 << 63;
                graphics::write_line(&mut screen, 0, p_right+p_size, 1);
                pixels[p_right+p_size] &= !(1 << 63);
                p_right -= 1;
            }
        }
        
        // Erase old ball
        let mut word = ball_pos.0 / 32;
        if !(ball_pos.0 == 0 && ball_pos.1 >= p_left - p_size && ball_pos.1 <= p_left + p_size) &&
            !(ball_pos.0 == WIDTH-1 && ball_pos.1 >= p_right - p_size && ball_pos.1 <= p_right + p_size) {
            pixels[ball_pos.1 as usize] &= !(1 << ball_pos.0);
        }
        graphics::write_line(&mut screen, (pixels[ball_pos.1 as usize] >> (32 * word)) as u32, ball_pos.1, word);

        // Update new ball position
        if ball_pos.0 as i8 + ball_vel.0 < 0 || ball_pos.0 as i8 + ball_vel.0 >= WIDTH as i8 {
            ball_vel.0 *= -1;
        }
        if ball_pos.1 as i8 + ball_vel.1 < 1 || ball_pos.1 as i8 + ball_vel.1 >= HEIGHT as i8 {
            ball_vel.1 *= -1;
        }
        ball_pos = ((ball_pos.0 as i8 + ball_vel.0) as usize, (ball_pos.1 as i8 + ball_vel.1) as usize);
        // if ball_pos.0 <= ball_vel.0 as usize { ball_pos.0 = 2 }
        // if ball_pos.0 >= WIDTH - 1 + ball_vel.0 as usize { ball_pos.0 = WIDTH-2 }

        // Draw new ball
        word = ball_pos.0 / 32;
        pixels[ball_pos.1 as usize] |= 1 << ball_pos.0;
        graphics::write_line(&mut screen, (pixels[ball_pos.1 as usize] >> (32 * word)) as u32, ball_pos.1, word);
        graphics::update(&mut screen);
    }
}
