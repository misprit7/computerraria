#![no_std]
#![no_main]

use tdriver::entry;
use tdriver::graphics;

entry!(main);

const BLOCK_SIZE: usize = 8;

const WIDTH: usize = 10 * BLOCK_SIZE;
const HEIGHT: usize = 10 * BLOCK_SIZE;

#[derive(Copy, Clone, PartialEq)]
enum Block {
    Empty,
    Dirt,
    Stone,
}

const START_WORLD: [[Block; WIDTH/BLOCK_SIZE]; HEIGHT/BLOCK_SIZE] = [
    [Block::Dirt, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Dirt],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty],
    [Block::Dirt, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Dirt],
];

fn render(  world: &[[Block; WIDTH/BLOCK_SIZE]; HEIGHT/BLOCK_SIZE],
            ppos: (usize, usize),
            pixels: &mut [u64; graphics::HEIGHT]) {
    let left = ppos.0 - graphics::WIDTH / 2;
    let right = ppos.1 - graphics::HEIGHT / 2;
    for y in 0..graphics::HEIGHT {
        pixels[y] = 0;
        for x in 0..graphics::WIDTH {
            if world[(left+x)/BLOCK_SIZE][(right+y)/BLOCK_SIZE] != Block::Empty {
                pixels[y] |= 1 << x;
            }
        }
    }
}

fn main() -> ! {
    let mut pixels: [u64; graphics::HEIGHT] = [0x0000000000000000; graphics::HEIGHT];
    let mut screen = graphics::init();

    let mut world: [[Block; WIDTH / BLOCK_SIZE]; HEIGHT / BLOCK_SIZE] = 
        START_WORLD;
    let mut ppos: (usize, usize) = (WIDTH/2, HEIGHT/2);

    loop {
        let input = graphics::input(&mut screen);
        if input & graphics::input_flags::LEFT != 0 { ppos.0 -= 1; }
        if input & graphics::input_flags::RIGHT != 0 { ppos.0 += 1; }
        if input & graphics::input_flags::DOWN != 0 { ppos.0 -= 1; }
        if input & graphics::input_flags::UP != 0 { ppos.0 += 1; }

        render(&world, ppos, &mut pixels);
        graphics::write_long(&mut screen, &pixels);
        graphics::update(&mut screen);
    }
}
