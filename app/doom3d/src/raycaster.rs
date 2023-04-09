use core::cmp::{min, max};
use tdriver::graphics;
use tdriver::graphics::HEIGHT;
use tdriver::graphics::WIDTH;

const PI: f32 = 3.1415926;

pub const MAP_WIDTH: usize = 24;
pub const MAP_HEIGHT: usize = 24;
pub const STEP_SIZE: f32 = 0.001;

pub struct Raycaster {
    map: [[bool; MAP_WIDTH]; MAP_HEIGHT],
    tan_half_fov: f32,
}

#[derive(Debug, PartialEq)]
enum WallType {
    Horizontal, // Normal on y-axis
    Vertical, // Normal on x-axis
    None, // No wall (looking into void)
}

impl Raycaster {
    pub fn new(map: [[bool; MAP_WIDTH]; MAP_HEIGHT], fov_deg: f32) -> Self {
        let fov_rad = fov_deg * PI / 180.0;
        Raycaster {
            map,
            tan_half_fov: (fov_rad / 2.0).tan(),
        }
    }

    pub fn render(&self, start_x: f32, start_y: f32, cam_angle_rad: f32, pixels: &mut [[bool; graphics::WIDTH]; graphics::HEIGHT]) {
        let mut last_wall_type = WallType::None;
        let mut last_top = 0;
        let mut last_bottom = 0;

        for x_pixel in 0..graphics::WIDTH {
            let screen_coord = (x_pixel as f32) / (graphics::WIDTH as f32);
            let ray_angle_rad = self.screen_coord_to_angle_rad(screen_coord) + cam_angle_rad;
            let hit = self.cast_ray(start_x, start_y, ray_angle_rad);

            if let Some((distance, wall_type)) = hit {
                let height: i32 = (graphics::HEIGHT as f32 / distance) as i32;
                let top: usize = min((graphics::HEIGHT as i32 / 2) + (height / 2), graphics::HEIGHT as i32 - 1) as usize;
                let bottom: usize = max((graphics::HEIGHT as i32 / 2) - (height / 2), 0) as usize;
                
                if wall_type != last_wall_type && x_pixel > 0 {
                    // hit a wall corner, draw a vertical line
                    //println!("Hit wall corner");

                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = if y_pixel >= bottom && y_pixel <= top {
                            true
                        } else {
                            false
                        };
                    }
                } else {
                    // hit a wall, draw top and bottom only
                    //println!("Hit wall but not corner");
                    
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = false;
                    }

                    pixels[top][x_pixel] = true;
                    pixels[bottom][x_pixel] = true;
                }
                
                //println!("Hit data! Wall type: {:?}", wall_type);

                last_wall_type = wall_type;
                last_top = top;
                last_bottom = bottom;
            }
            else {
                if last_wall_type != WallType::None {
                    // hit blank space after seeing wall, draw a vertical line
                    //println!("Hit blank space first time, draw corner");
                    
                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = if y_pixel >= last_bottom && y_pixel <= last_top {
                            true
                        } else {
                            false
                        };
                    }
                }
                else {
                    // hit blank space, draw nothing
                    //println!("Hit blank space, draw nothing");

                    for y_pixel in 0..graphics::HEIGHT {
                        pixels[y_pixel][x_pixel] = false;
                    }
                }

                last_wall_type = WallType::None;
            }
        }
    }
    
    fn screen_coord_to_angle_rad(&self, screen_coord: f32) -> f32 {
        // ooh magic
        ((2.0 * screen_coord - 1.0) * self.tan_half_fov).atan()
    }

    fn cast_ray(&self, start_x: f32, start_y: f32, ray_angle_rad: f32) -> Option<(f32, WallType)> {
        let dir_x = ray_angle_rad.cos();
        let dir_y = ray_angle_rad.sin();

        let mut pos_x = start_x;
        let mut pos_y = start_y;

        loop {
            if let Some((idx_x, idx_y)) = self.loc_to_idx(pos_x, pos_y) {
                if self.map[idx_y][idx_x] {
                    // ray has collided with a wall
                    let dx = pos_x - start_x;
                    let dy = pos_y - start_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    let x_diff = (pos_x - pos_x.round()).abs();
                    let y_diff = (pos_y - pos_y.round()).abs();
                    let wall_type = if x_diff < y_diff {
                        WallType::Vertical  
                    } else {
                        WallType::Horizontal
                    };

                    return Some((distance, wall_type));
                }
            }
            else {
                // ray has left map area
                return None;
            }

            pos_x += dir_x * STEP_SIZE;
            pos_y += dir_y * STEP_SIZE;
        }
    }

    fn loc_to_idx(&self, pos_x: f32, pos_y: f32) -> Option<(usize, usize)> {
        let idx_x = pos_x as usize;
        let idx_y = pos_y as usize;

        if [
            pos_x < 0.0,
            pos_y < 0.0,
            idx_x >= MAP_WIDTH,
            idx_y >= MAP_HEIGHT,
        ].iter().any(|&x| x) {
            None
        }
        else {
            Some((idx_x, idx_y))
        }
    }
}
