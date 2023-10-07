#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;
use tiles::SHAPES;

mod tiles;

entry!(main);

const TOP: usize = 7;
const MIDDLE: usize = 30;


// Entry point of user code

fn create_board() -> [u64; graphics::HEIGHT] {
    let mut base_board : [u64; graphics::HEIGHT] = [0;graphics::HEIGHT];
    for i in 0..8{
        base_board[i] |= u64::MAX;
    }
    for i in 8..40{
        base_board[i] |= 0xFFFFFF0000FFFFFF;
    }
    for i in 40..48{
        base_board[i] |= u64::MAX;
    }
    base_board
}

fn find_bottom(curr_pos: &[u64;4]) -> usize{
    for i in 0..4{
        if curr_pos[3-i] != 0{
            return 3-i
        }
    }
    return 0;
}

fn find_top(curr_pos: &[u64;4]) -> usize{
    for i in 0..4{
        if curr_pos[i] != 0{
            return i
        }
    }
    return 0;
}

fn get_rand(x: usize)->usize{
    ((690697*x+1)%3_usize.pow(4)).into()
}

fn clear_tiles(curr_board: &mut [u64; graphics::HEIGHT]) {
    let mut row_index = 8;

    while row_index < 40 {
        if curr_board[row_index] == u64::MAX {
            for j in (8..row_index).rev() {
                curr_board[j + 1] = curr_board[j];
            }
            curr_board[8] = 0;
        } else {
            row_index += 1;
        }
    }
}

fn main() -> ! {
    let base_board = create_board();
    let mut curr_board: [u64; graphics::HEIGHT] = create_board();
    let mut screen = graphics::init();

    let mut new_tile = true;

    let mut shape: [[u64;4];4] = [[0;4];4];
    let mut a = tiles::I[1];
    let mut new_pos: [u64;4] = [0;4];
    let mut curr_pos: [u64;4] = [0;4];
    let mut n = find_bottom(&a);


    let mut r = TOP + n;
    let mut tile_pos = MIDDLE;
    let mut rot: usize = 0;
    let mut rand = 67;


    loop {
        graphics::write_long(&mut screen, &curr_board);
        graphics::update(&mut screen);

        if (curr_board[r+n] & curr_pos[n]) > 0{
            if r == TOP-find_top(&a)+2{
                curr_board = [u64::MAX;graphics::HEIGHT];
                continue;
            }
            new_tile = true;
        }
        if ((!curr_pos[n] & curr_board[r+n-1]) & curr_pos[n-1])>0{
            if r == TOP-find_top(&a)+2{
                curr_board = [u64::MAX;graphics::HEIGHT];
                continue;
            }
            new_tile = true;
        }

        if new_tile{
            //clear board
            clear_tiles(&mut curr_board);

            rand = get_rand(rand);
            shape = SHAPES[rand%7];
            rand = get_rand(rand);
            rot = rand%4;
            a = shape[rot];
            tile_pos = MIDDLE;
            n = find_bottom(&new_pos);
            r = TOP-find_top(&a)+1;

            for i in 0..4{
                new_pos[i] = a[i]<<MIDDLE;
            }

            new_tile = false;
        }else{
            for i in 0..4{
                curr_board[i+r-1] = !curr_pos[i] & curr_board[i+r-1];
            }

            let input = graphics::input(&mut screen);
            let up = input & graphics::input_flags::UP != 0;
            let down = input & graphics::input_flags::DOWN != 0;
            let left = input & graphics::input_flags::LEFT != 0;
            let right = input & graphics::input_flags::RIGHT != 0;
    
            if left{
                tile_pos -= 1;
                for i in 0..4{
                    new_pos[i] = curr_pos[i]>>1;
                }
            }
            else if right {
                tile_pos += 1;
                for i in 0..4{
                    new_pos[i] = curr_pos[i]<<1;
                }
            }
            else if up {
                if !(rot == 3){
                    rot += 1;
                }
                else{
                    rot = 0;
                }
                a = shape[rot];
                for i in 0..4{
                    new_pos[i] = a[i]<<tile_pos;
                }
            }
            else if down {
                if !(rot == 0){
                    rot -= 1;
                }
                else{
                    rot = 3;
                }
                a = shape[rot];
                for i in 0..4{
                    new_pos[i] = a[i]<<tile_pos;
                }
            }
    
            //check for conflicts with current board and new tile
            for i in 0..4{
                if (curr_board[i+r] & new_pos[i]) != 0{
                    new_pos = curr_pos;
                }
            }
        }



        //draw new tile on curr_board
        curr_pos = new_pos;
        n = find_bottom(&curr_pos);
        for i in 0..4{
            curr_board[i+r] = curr_pos[i] | curr_board[i+r];
        }
        for i in 0..graphics::HEIGHT{
            curr_board[i] = curr_board[i] | base_board[i];
        }

        if r<40{
            r+=1;
        }

        
    }
}
