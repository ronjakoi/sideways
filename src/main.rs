use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use rand::prelude::*;
use sdl2::rect::{Rect};
use sdl2::render::Texture;
use std::collections::HashSet;
use std::time::{Duration, Instant};

mod player;
mod starfield;
mod enemy;

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub enum Axis {
    X,
    Y,
}

struct Projectile<'a, 'b> {
    pub v: Velocity,
    pub x: i32,
    pub y: i32,
    pub damage: i32,
    pub sprite: &'a Texture<'b>,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Velocity {
        Velocity { x, y }
    }

    pub fn apply_inertia(&mut self, axis: Axis) {
        const INERTIA: i32 = 3;

        match axis {
            Axis::Y => {
                if self.y == -1 {
                    self.y = 0;
                } else if self.y < 0 {
                    self.y += INERTIA;
                } else if self.y == 1 {
                    self.y = 0;
                } else if self.y > 0 {
                    self.y -= INERTIA;
                }
            }
            Axis::X => {
                if self.x == -1 {
                    self.x = 0;
                } else if self.x < 0 {
                    self.x += INERTIA;
                } else if self.x == 1 {
                    self.x = 0;
                } else if self.x > 0 {
                    self.x -= INERTIA;
                }
            }
        }
    }
}

impl std::ops::Add for Velocity {
    type Output = Self;
    fn add(self, other: Self) -> Velocity {
        Velocity {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Velocity {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const HEIGHT: u32 = 384;
const WIDTH: u32 = 512;
const PLAYER_MAX_SPEED: i32 = 6;
const PLAYER_PROJECTILE_SPEED: i32 = 10;
const SHOOT_DELAY: u64 = 80; // milliseconds
const ENEMY_SPAWN_CHANCE: f64 = 0.6;

// Read keyboard input
//
// Move the ship with arrow keys
// Shoot with S or Space
//
// Keep track of projectile shooting delay
fn handle_input<'a, 'b>(
    keycodes: &HashSet<Keycode>,
    player: &mut player::Player,
    projectiles: &mut Vec<Projectile<'a, 'b>>,
    projectile_texture: &'a Texture<'b>,
    last_shot: &mut Option<Instant>,
) {
    const SPEED: i32 = 2; // how much to increment player ship velocity each frame
    if !(keycodes.contains(&Keycode::Up) || keycodes.contains(&Keycode::Down)) {
        player.v.apply_inertia(Axis::Y)
    }
    if !(keycodes.contains(&Keycode::Left) || keycodes.contains(&Keycode::Right)) {
        player.v.apply_inertia(Axis::X)
    }

    for k in keycodes {
        match k {
            Keycode::Up => player.v += Velocity::new(0, -SPEED),
            Keycode::Down => player.v += Velocity::new(0, SPEED),
            Keycode::Left => player.v += Velocity::new(-SPEED, 0),
            Keycode::Right => player.v += Velocity::new(SPEED, 0),
            Keycode::S | Keycode::Space => {
                let now = Instant::now();
                if (last_shot.is_some()
                    && now - last_shot.unwrap() > Duration::from_millis(SHOOT_DELAY))
                    || last_shot.is_none()
                {
                    projectiles.push(Projectile {
                        v: Velocity {
                            x: PLAYER_PROJECTILE_SPEED,
                            y: 0,
                        },
                        x: player.x + player.width as i32,
                        y: player.y + (player.height / 2) as i32,
                        damage: 10,
                        sprite: &projectile_texture,
                    });
                    *last_shot = Some(now);
                }
            }
            _ => {}
        };
    }

    // limit player ship's maximum speed
    if player.v.x > PLAYER_MAX_SPEED {
        player.v.x = PLAYER_MAX_SPEED;
    } else if player.v.x < -PLAYER_MAX_SPEED {
        player.v.x = -PLAYER_MAX_SPEED;
    }
    if player.v.y > PLAYER_MAX_SPEED {
        player.v.y = PLAYER_MAX_SPEED;
    } else if player.v.y < -PLAYER_MAX_SPEED {
        player.v.y = -PLAYER_MAX_SPEED;
    }
}

fn advance_projectiles(projectiles: &mut Vec<Projectile>) {
    for p in projectiles.into_iter() {
        p.x += p.v.x;
    }
    // delete projectiles which have gone off the
    // right-hand edge of the screen
    projectiles.retain(|p| p.x < WIDTH as i32);
}

fn draw_projectiles(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    projectiles: &[Projectile],
) -> Result<(), String> {
    for proj in projectiles {
        let rect = Rect::new(
            proj.x,
            proj.y,
            proj.sprite.query().width,
            proj.sprite.query().height,
        );
        if canvas.copy(proj.sprite, None, rect).is_err() {
            return Err(String::from("Could not draw projectile"));
        };
    }
    Ok(())
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

    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let texture_creator = canvas.texture_creator();

    let mut rng = thread_rng();

    let mut starfield = starfield::Starfield::new();

    let player_ship = texture_creator.load_texture("assets/playership.png")?;
    let player_shot = texture_creator.load_texture("assets/playershot.png")?;
    let enemy_ship = texture_creator.load_texture("assets/enemyship.png")?;
    let mut player = player::Player::from_sprite(&player_ship);

    let mut player_projectiles: Vec<Projectile> = vec![];
    let mut last_shot: Option<Instant> = None;

    let mut enemies: Vec<enemy::Enemy> = vec![];
    // check once every second whether to spawn new enemy
    let mut enemy_tick = Instant::now();
    let mut one_second = Duration::from_secs(1);

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

        let pressed_keys: HashSet<_> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        handle_input(
            &pressed_keys,
            &mut player,
            &mut player_projectiles,
            &player_shot,
            &mut last_shot,
        );
        player.apply_velocity();

        starfield.spawn_new_stars();

        starfield.draw(&mut canvas)?;
        starfield.advance();

        let now = Instant::now();
        if now - enemy_tick >= one_second && rng.gen::<f64>() < ENEMY_SPAWN_CHANCE {
            enemies.push(enemy::Enemy::from_sprite(&enemy_ship));
            enemy_tick = now;
        }

        for enemy in &mut enemies {
            enemy.draw(&mut canvas)?;
            enemy.advance();
        }

        player.draw(&mut canvas)?;
        draw_projectiles(&mut canvas, &player_projectiles)?;
        advance_projectiles(&mut player_projectiles);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 50));
    }
    Ok(())
}
