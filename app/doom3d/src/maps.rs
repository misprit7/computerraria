use crate::raycaster::{MAP_WIDTH, MAP_HEIGHT};

// pub const MAP: [[bool; MAP_WIDTH]; MAP_HEIGHT] =
//     [[true, true, true], 
//     [true, false, true],
//     [true, true, true]];

pub const MAP: [[bool; MAP_WIDTH]; MAP_HEIGHT] =
    [[true, true, true, true, true], 
    [true, true, false, true, true], 
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, true, false, true, true],
    [true, true, true, true, true]];

// pub const MAP: [[bool; MAP_WIDTH]; MAP_HEIGHT] = [
//     [true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,true,true,true,true,true,false,false,false,false,true,false,true,false,true,false,false,false,true],
//     [true,false,false,false,false,false,true,false,false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,true,false,false,false,true,false,false,false,false,true,false,false,false,true,false,false,false,true],
//     [true,false,false,false,false,false,true,false,false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,true,true,false,true,true,false,false,false,false,true,false,true,false,true,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,true,true,true,true,true,true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,false,true,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,false,false,false,false,true,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,false,true,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,false,true,true,true,true,true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,true,true,true,true,true,true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true],
//     [true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true]
// ];