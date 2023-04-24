use std::fs::File;
use std::io::prelude::*;
use tdriver::graphics;

const PI: f32 = 3.14159;
const FOV_DEG: f32 = 60.0;
const FOV_RAD: f32 = FOV_DEG * PI / 180.0;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("pixel_to_ray_angle_lookup.rs");
    let mut file = File::create(&dest_path).unwrap();
    write!(
        file,
        "pub const PIXEL_TO_RAY_ANGLE_TABLE: [I5F11; {}] = [\n",
        graphics::WIDTH
    )
    .unwrap();

    let tan_half_fov = (FOV_RAD / 2.0).tan();
    for x_pixel in 0..graphics::WIDTH {
        // Create a lookup table to map x pixel index to the ray angle pointing in the direction of that pixel, in rad
        let screen_coord = x_pixel as f32 / graphics::WIDTH as f32;
        let angle_rad = ((2.0 * screen_coord - 1.0) * tan_half_fov).atan();

        // Couldn't find a const fn to make a fixed point from a f32, but for some reason there's one for a string
        write!(file, "   I5F11::unwrapped_from_str(\"{}\"),\n", angle_rad).unwrap();
    }

    write!(file, "];\n").unwrap();
}
