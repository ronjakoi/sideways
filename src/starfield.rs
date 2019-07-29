use crate::Velocity;
use crate::{HEIGHT, WIDTH};
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas};
use sdl2::video::Window;

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
        let mut sf = Starfield {
            stars: [None; MAX_STARS],
        };
        sf.spawn(true);
        sf
    }

    // Spawn new stars to fill up the starfield
    // If first_frame == true, spawn stars randomly on the x axis
    // as well as the y axis. Otherwise spawn them on the right edge of the screen,
    // i.e. x == WIDTH.
    fn spawn(&mut self, first_frame: bool) {
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

    pub fn spawn_new_stars(&mut self) {
        self.spawn(false);
    }

    pub fn advance(&mut self) {
        for star in self.stars.iter_mut() {
            match star {
                None => {}
                Some(s) => {
                    // If this star has gone off the left edge of the screen,
                    // reset it
                    if s.x < 0 {
                        *star = None;
                        continue;
                    }
                    s.x += s.v.x;
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for s in self.stars.iter().filter_map(|&x| x) {
            canvas.set_draw_color(s.color);
            if canvas.draw_point(Point::new(s.x, s.y)).is_err() {
                return Err(String::from("Could not draw stars"));
            }
        }
        Ok(())
    }
}
