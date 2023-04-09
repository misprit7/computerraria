use core::cmp::{max, min};
use tdriver::graphics;
use libm;

const PI: f32 = 3.1415926;
pub const MAP_WIDTH: usize = 5;
pub const MAP_HEIGHT: usize = 7;

pub struct Raycaster {
    map: [[bool; MAP_WIDTH]; MAP_HEIGHT],
    tan_half_fov: f32,
}

struct RayHit {
    length: f32,
    idx_x: usize,
    idx_y: usize,
    normal_along_x: bool,
}

impl Raycaster {
    pub fn new(map: [[bool; MAP_WIDTH]; MAP_HEIGHT], fov_deg: f32) -> Self {
        let fov_rad = fov_deg * PI / 180.0;
        Raycaster {
            map,
            tan_half_fov: libm::tanf(fov_rad / 2.0),
        }
    }

    pub fn render(
        &self,
        start_x: f32,
        start_y: f32,
        cam_angle_rad: f32,
        pixels: &mut [[bool; graphics::WIDTH]; graphics::HEIGHT],
    ) {
        let mut last_hit = RayHit {
            length: 0.0,
            idx_x: 0,
            idx_y: 0,
            normal_along_x: false,
        };
        let mut last_top = 0;
        let mut last_bottom = 0;
        let mut last_on_wall = false;

        for x_pixel in 0..graphics::WIDTH {
            let screen_coord = (x_pixel as f32) / (graphics::WIDTH as f32);
            let ray_angle_rad = self.screen_coord_to_angle_rad(screen_coord) + cam_angle_rad;
            let hit = self.cast_ray(start_x, start_y, ray_angle_rad);

            if let Some(hit) = hit {
                let height: i32 = (graphics::HEIGHT as f32 / hit.length) as i32;
                let top: usize = min(
                    (graphics::HEIGHT as i32 / 2) + (height / 2),
                    graphics::HEIGHT as i32 - 1,
                ) as usize;
                let bottom: usize = max((graphics::HEIGHT as i32 / 2) - (height / 2), 0) as usize;

                let adjacent_x = (hit.idx_x as i32 - last_hit.idx_x as i32).abs() <= 1;
                let adjacent_y = (hit.idx_y as i32 - last_hit.idx_y as i32).abs() <= 1;

                if (!adjacent_x
                    || !adjacent_y
                    || hit.normal_along_x != last_hit.normal_along_x
                    || !last_on_wall)
                    && x_pixel > 0
                {
                    // hit a wall corner, draw a vertical line
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = y_pixel >= bottom && y_pixel <= max(top, last_top);
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

    fn screen_coord_to_angle_rad(&self, screen_coord: f32) -> f32 {
        // ooh magic
        libm::atanf((2.0 * screen_coord - 1.0) * self.tan_half_fov)
    }

    // DDA algorithm (https://www.youtube.com/watch?v=NbSee-XM7WA&ab_channel=javidx9)
    fn cast_ray(&self, start_x: f32, start_y: f32, ray_angle_rad: f32) -> Option<RayHit> {
        let dir_x = libm::cosf(ray_angle_rad);
        let dir_y = libm::sinf(ray_angle_rad);
        let ray_unit_step_size_x = libm::fabsf(1.0 / dir_x); // Length of step if moving 1 unit in x
        let ray_unit_step_size_y = libm::fabsf(1.0 / dir_y); // Length of step if moving 1 unit in y

        // Length of ray if next step is 1 unit in x (account for off-grid start)
        let mut ray_length_x = if dir_x >= 0.0 {
            libm::ceilf(start_x) - start_x
        } else {
            start_x - libm::floorf(start_x)
        } * ray_unit_step_size_x;

        // Length of ray if next step is 1 unit in y (account for off-grid start)
        let mut ray_length_y = if dir_y >= 0.0 {
            libm::ceilf(start_y) - start_y
        } else {
            start_y - libm::floorf(start_y)
        } * ray_unit_step_size_y;

        let step_x = if dir_x > 0.0 {1} else {-1};
        let step_y = if dir_y > 0.0 {1} else {-1};
        let mut idx_x = start_x as i32;
        let mut idx_y = start_y as i32;

        let mut ray_length = 0.0;
        loop {
            if self.valid_map_idx(idx_x, idx_y) {
                if self.map[idx_y as usize][idx_x as usize] {
                    // ray has collided with a wall!
                    let ray_hit_x = ray_length * dir_x;
                    let ray_hit_y = ray_length * dir_y;

                    let x_diff = libm::fabsf(ray_hit_x - libm::roundf(ray_hit_x));
                    let y_diff = libm::fabsf(ray_hit_y - libm::roundf(ray_hit_y));
                    let normal_along_x = x_diff < y_diff;

                    return Some(RayHit {
                        length: ray_length,
                        idx_x: idx_x as usize,
                        idx_y: idx_y as usize,
                        normal_along_x,
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
            } else {
                idx_y += step_y;
                ray_length = ray_length_y;
                ray_length_y += ray_unit_step_size_y;
            }
        }
    }

    fn valid_map_idx(&self, pos_x: i32, pos_y: i32) -> bool {
        pos_x >= 0 && pos_y >= 0 && pos_x < MAP_WIDTH as i32 && pos_y < MAP_HEIGHT as i32
    }
}
