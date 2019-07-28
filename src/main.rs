use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::cmp;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Star {
    pub color: Color,
    pub speed: i32,
    pub x: i32,
    pub y: i32,
}

const HEIGHT: u32 = 384;
const WIDTH: u32 = 512;
const MAX_STARS: usize = 128;

// How many free star slots are there?
fn count_free_stars(stars: &[Option<Star>]) -> u32 {
    stars.iter().filter(|x| x.is_none()).count() as u32
}

// Spawn a random number of new stars in the star field.
// If first_frame == true, spawn stars randomly on the x axis
// as well as the y axis. Otherwise spawn them on the right edge of the screen,
// i.e. y == WIDTH.
fn spawn_new_stars(stars: &mut [Option<Star>], first_frame: bool) -> () {
    const SPEED_MIN: i32 = 3;
    const SPEED_MAX: i32 = 15;
    const STARS_PER_FRAME_MIN: u32 = 0;
    const STARS_PER_FRAME_MAX: u32 = 10;

    let mut spawned_stars = 0;
    let mut i = 0;
    let mut rng = thread_rng();
    let free_stars = count_free_stars(&stars);
    if free_stars == 0 {
        return;
    }

    let new_stars = if first_frame {
        MAX_STARS as u32
    } else {
        rng.gen_range(
            STARS_PER_FRAME_MIN,
            cmp::min(free_stars, STARS_PER_FRAME_MAX + 1),
        )
    };
    while i < MAX_STARS && spawned_stars < new_stars {
        match stars[i] {
            None => {
                stars[i] = Some(Star {
                    color: Color::RGB(200, 200, 200),
                    speed: rng.gen_range(SPEED_MIN, SPEED_MAX + 1),
                    x: if first_frame {
                        rng.gen_range(0, WIDTH as i32)
                    } else {
                        WIDTH as i32 - 1
                    },
                    y: rng.gen_range(0, HEIGHT as i32),
                });
                spawned_stars += 1;
            }
            Some(_) => {}
        }
        i += 1;
    }
}

fn advance_stars(stars: &mut [Option<Star>]) -> () {
    for star in stars.iter_mut() {
        match star {
            None => {}
            Some(s) => {
                // If this star has gone off the left edge of the screen,
                // reset it
                if s.x < 0 {
                    *star = None;
                    continue;
                }

                s.x -= s.speed;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // set up the draw window, scaled up 2x from actual game resolution
    // for a chunkier retro effect
    let window = video
        .window("Sideways", WIDTH * 2, HEIGHT * 2)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    // A buffer of Option<Star>
    // If an element is None, that slot in the buffer is available for
    // spawning a new star.
    let mut stars: [Option<Star>; MAX_STARS] = [None; MAX_STARS];

    // for the first frame we need to draw the star field differently
    let mut first_frame = true;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Blank the window
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        spawn_new_stars(&mut stars, first_frame);

        for s in stars.iter().filter_map(|&x| x) {
            canvas.set_draw_color(s.color);
            canvas
                .draw_point(Point::new(s.x, s.y))
                .or(Err("Unable to draw star\n"))?;
        }
        advance_stars(&mut stars);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        first_frame = false;
    }
    Ok(())
}
