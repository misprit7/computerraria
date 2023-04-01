#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;
use tdriver::graphics::HEIGHT;

entry!(main);

fn update_board(board: &[u64; graphics::HEIGHT]) -> [u64; graphics::HEIGHT] {
    let mut board_next: [u64; graphics::HEIGHT] = [0; graphics::HEIGHT];
    for r in 0..graphics::HEIGHT {
        // let c = 1;
        for c in 0..graphics::WIDTH {
            let cur_state = (board[r] >> c) & 0b1;
            let start_i = if r > 0 {r-1} else {r};
            let end_i = if r + 1 < board.len() {r+1} else {r};

            let start_j = if c > 0 {c-1} else {c};
            let end_j = if c + 1 < 64 {c+1} else {c};

            let mut neighbors_alive = 0;
            for i in start_i..=end_i {
                for j in start_j..=end_j {
                    if i == r && j == c { continue }
                    neighbors_alive += (board[i] >> j) & 0b1;
                }
            }
            if cur_state == 1 {
                if neighbors_alive < 2 || neighbors_alive > 3 {
                    board_next[r] &= !(0b1 << c);
                } else {
                    board_next[r] |= 0b1 << c;
                }
            } else {
                if neighbors_alive == 3 {
                    board_next[r] |= 0b1 << c;
                } else {
                    board_next[r] &= !(0b1 << c);
                }
            }
        }
    }
    board_next
}

// Entry point of user code
fn main() -> ! {
    let mut board: [u64; graphics::HEIGHT] = [
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000100000100,
        0b0000000000000000000000000000000000000000000000000000000100011000,
        0b0000000000000000000000000000000000000000000000000000000100001100,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
        0b0000000000000000000000000000000000000000000000000000000000000000,
    ];
    let mut screen = graphics::init();
    graphics::write_long(&mut screen, &board);
    graphics::update(&mut screen);
    loop {
        // for i in 0..48 {
        //     for j in 0..64 {
        //         board[i] = 1 <<j;
        //         graphics::write_long(&mut screen, &board);
        //         graphics::update(&mut screen);
        //     }
        // }
        board = update_board(&board);
        graphics::write_long(&mut screen, &board);
        graphics::update(&mut screen);
    }
}
