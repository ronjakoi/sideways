use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use crate::{WIDTH, HEIGHT};
use crate::Velocity;

const MAX_STARS: usize = 128;

#[derive(Clone, Copy, Debug)]
struct Star {
    pub color: Color,
    pub x: i32,
    pub y: i32,
    pub v: Velocity,
}

pub struct Starfield {
    stars: [Option<Star>; MAX_STARS],
}

impl Starfield {
    pub fn new() -> Self {
        Starfield { stars: [None; MAX_STARS] }
    }

    // Spawn new stars to fill up the starfield
    // If first_frame == true, spawn stars randomly on the x axis
    // as well as the y axis. Otherwise spawn them on the right edge of the screen,
    // i.e. y == WIDTH.
    pub fn spawn(&mut self, first_frame: bool) {
        const SPEED_MIN: i32 = 3;
        const SPEED_MAX: i32 = 15;

        let mut rng = thread_rng();
        for s in self.stars.iter_mut().filter(|x| x.is_none()) {
            *s = Some(Star {
                color: Color::RGB(200, 200, 200),
                v: Velocity::new(-rng.gen_range(SPEED_MIN, SPEED_MAX + 1), 0),
                x: if first_frame {
                    rng.gen_range(0, WIDTH as i32)
                } else {
                    WIDTH as i32 - 1
                },
                y: rng.gen_range(0, HEIGHT as i32),
            });
        }

    }

    pub fn advance(&mut self) {
        dbg!(self.stars[0]);
        for i in 0..self.stars.len() {
            match self.stars[i] {
                None => {},
                Some(mut star) => {
                    // If this star has gone off the left edge of the screen,
                    // reset it
                    if star.x < 0 {
                        self.stars[i] = None;
                        continue;
                    }
                    let old_x = star.x;
                    star.x += star.v.x;
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for s in self.stars.iter().filter_map(|&x| x) {
            canvas.set_draw_color(s.color);
            if canvas
                .draw_point(Point::new(s.x, s.y)).is_err() {
                    return Err(String::from("Could not draw stars"));
                }
        }
        Ok(())
    }
}
