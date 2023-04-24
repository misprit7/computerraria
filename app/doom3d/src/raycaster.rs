use core::cmp::{max, min};
use tdriver::graphics;
use crate::maps::{MAP_WIDTH, MAP_HEIGHT};
use fixed::types::{I9F7, I5F11, I16F0, I16F16};
use cordic;

use fixed::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pixel_to_ray_angle_lookup.rs"));

pub struct Raycaster {
    map: [[bool; MAP_WIDTH]; MAP_HEIGHT],
}

struct RayHit {
    length: I5F11,
    idx_x: usize,
    idx_y: usize,
    normal_along_x: bool,
}

impl Raycaster {
    pub fn new(map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> Self {
        Raycaster {
            map,
        }
    }

    pub fn render(
        &self,
        start_x: I5F11,
        start_y: I5F11,
        cam_angle_rad: I5F11,
        pixels: &mut [[bool; graphics::WIDTH]; graphics::HEIGHT],
    ) {
        let mut last_hit = RayHit {
            length: I5F11::const_from_int(0),
            idx_x: 0,
            idx_y: 0,
            normal_along_x: false,
        };
        let mut last_top = 0;
        let mut last_bottom = 0;
        let mut last_on_wall = false;

        for x_pixel in 0..graphics::WIDTH {
            let ray_angle_rad = PIXEL_TO_RAY_ANGLE_TABLE[x_pixel] + cam_angle_rad;
            let hit = self.cast_ray(start_x, start_y, ray_angle_rad);

            if let Some(hit) = hit {
                let height: i32 = (graphics::HEIGHT.to_fixed::<I9F7>().wide_div(hit.length)).to_num();
                let top: usize = min(
                    (graphics::HEIGHT as i32 / 2) + (height / 2),
                    graphics::HEIGHT as i32 - 1,
                ) as usize;
                let bottom: usize = max((graphics::HEIGHT as i32 / 2) - (height / 2), 0) as usize;

                let x_diff = hit.idx_x.abs_diff(last_hit.idx_x);
                let y_diff = hit.idx_y.abs_diff(last_hit.idx_y);
                let diagonal = x_diff == 1 && y_diff == 1; 
                let far_away = x_diff >= 2 || y_diff >= 2; 
                let new_normal = hit.normal_along_x != last_hit.normal_along_x; 
                
                if (diagonal || far_away || new_normal || !last_on_wall) && x_pixel > 0 {
                    // hit a wall corner, draw a vertical line
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = y_pixel >= min(bottom, last_bottom) && y_pixel <= max(top, last_top);
                    }
                } else {
                    // hit a wall, draw top and bottom only
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = false;
                    }

                    pixels[top][x_pixel] = true;
                    pixels[bottom][x_pixel] = true;
                }

                last_hit = hit;
                last_top = top;
                last_bottom = bottom;
                last_on_wall = true;
            } else {
                if last_on_wall {
                    // hit blank space after seeing wall, draw a vertical line
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = y_pixel >= last_bottom && y_pixel <= last_top;
                    }
                } else {
                    // hit blank space, draw nothing
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = false;
                    }
                }

                last_on_wall = false;
            }
        }
    }

    // DDA algorithm (https://www.youtube.com/watch?v=NbSee-XM7WA&ab_channel=javidx9)
    fn cast_ray(&self, start_x: I5F11, start_y: I5F11, ray_angle_rad: I5F11) -> Option<RayHit> {
        let (dir_y, dir_x) = cordic::sin_cos(ray_angle_rad);
        let ray_unit_step_size_x = match dir_x.abs().checked_recip() {
            Some(v) => v,
            None => I5F11::MAX
        }; // Length of step if moving 1 unit in x
        let ray_unit_step_size_y = match dir_y.abs().checked_recip() {
            Some(v) => v,
            None => I5F11::MAX
        }; // Length of step if moving 1 unit in y
        let mut last_hit_x_normal = false; // Normal of last wall hit

        // Length of ray if next step is 1 unit in x (account for off-grid start)
        let mut ray_length_x = if dir_x >= 0.0 {
            last_hit_x_normal = true;
            start_x.ceil() - start_x
        } else {
            start_x - start_x.floor()
        } * ray_unit_step_size_x;

        // Length of ray if next step is 1 unit in y (account for off-grid start)
        let mut ray_length_y = if dir_y >= 0.0 {
            start_y.ceil() - start_y
        } else {
            start_y - start_y.floor()
        } * ray_unit_step_size_y;

        let step_x = if dir_x > 0.0 {1} else {-1};
        let step_y = if dir_y > 0.0 {1} else {-1};
        let mut idx_x = start_x.to_num::<i32>();
        let mut idx_y = start_y.to_num::<i32>();

        let mut ray_length = I5F11::const_from_int(0);
        loop {
            if self.valid_map_idx(idx_x, idx_y) {
                if self.map[idx_y as usize][idx_x as usize] {
                    // ray has collided with a wall!
                    return Some(RayHit {
                        length: ray_length,
                        idx_x: idx_x as usize,
                        idx_y: idx_y as usize,
                        normal_along_x: last_hit_x_normal,
                    });
                }
            } else {
                // ray has left map area, exit
                return None;
            }

            // Walk along shortest ray
            if ray_length_x <= ray_length_y {
                idx_x += step_x;
                ray_length = ray_length_x;
                ray_length_x += ray_unit_step_size_x;
                last_hit_x_normal = true;
            } else {
                idx_y += step_y;
                ray_length = ray_length_y;
                ray_length_y += ray_unit_step_size_y;
                last_hit_x_normal = false;
            }
        }
    }

    fn valid_map_idx(&self, pos_x: i32, pos_y: i32) -> bool {
        pos_x >= 0 && pos_y >= 0 && pos_x < MAP_WIDTH as i32 && pos_y < MAP_HEIGHT as i32
    }
}
