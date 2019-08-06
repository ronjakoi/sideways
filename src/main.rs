use rand::prelude::*;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::collections::HashSet;
use std::time::{Duration, Instant};

mod collide;
mod enemy;
mod player;
mod projectile;
mod starfield;

use collide::Collider;
use projectile::{LethalTo, ProjShape, Projectile};

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

pub enum Axis {
    X,
    Y,
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Velocity {
        Velocity { x, y }
    }

    pub fn apply_inertia(&mut self, axis: Axis) {
        const INERTIA: f64 = 0.7;
        match axis {
            Axis::Y => {
                self.y *= INERTIA;
            }
            Axis::X => {
                self.x *= INERTIA;
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
const PLAYER_MAX_SPEED: f64 = 6.0;
const SHOOT_DELAY: u64 = 80; // milliseconds
const PLAYER_PROJECTILE_SPEED: f64 = 10.0;
const ENEMY_SPAWN_CHANCE: f64 = 0.2;
const ENEMY_PROJECTILE_SPEED: f64 = 4.5;

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
    const SPEED: f64 = 2.0; // how much to increment player ship velocity each frame
    if !(keycodes.contains(&Keycode::Up) || keycodes.contains(&Keycode::Down)) {
        player.v.apply_inertia(Axis::Y)
    }
    if !(keycodes.contains(&Keycode::Left) || keycodes.contains(&Keycode::Right)) {
        player.v.apply_inertia(Axis::X)
    }

    for k in keycodes {
        match k {
            Keycode::Up => player.v += Velocity::new(0.0, -SPEED),
            Keycode::Down => player.v += Velocity::new(0.0, SPEED),
            Keycode::Left => player.v += Velocity::new(-SPEED, 0.0),
            Keycode::Right => player.v += Velocity::new(SPEED, 0.0),
            Keycode::S | Keycode::Space => {
                let now = Instant::now();
                if (last_shot.is_some()
                    && now - last_shot.unwrap() > Duration::from_millis(SHOOT_DELAY))
                    || last_shot.is_none()
                {
                    projectiles.push(Projectile::from_sprite(
                        projectile_texture,
                        &player.shape,
                        PLAYER_PROJECTILE_SPEED,
                        ProjShape::Rectangle,
                        LethalTo::Enemy,
                        None,
                    ));
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
    let enemy_shot = texture_creator.load_texture("assets/enemy_projectile.png")?;
    let enemy_ship = texture_creator.load_texture("assets/enemyship.png")?;
    let mut player = player::Player::from_sprite(&player_ship);

    let mut projectiles: Vec<Projectile> = Vec::with_capacity(128);
    let mut last_shot: Option<Instant> = None;

    let mut enemies: Vec<enemy::Enemy> = vec![];
    // check once every second whether to spawn new enemy
    let mut enemy_tick = Instant::now();
    let one_second = Duration::from_secs(1);

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
            &mut projectiles,
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
            let mut hit_by_proj_idx: Option<usize> = None;
            for (i, proj) in projectiles.iter_mut().enumerate() {
                if !proj.lethal_to_enemy() {
                    continue;
                }
                if enemy.collide(&proj.shape) {
                    enemy.die();
                    hit_by_proj_idx = Some(i);
                    break;
                }
            }
            if let Some(i) = hit_by_proj_idx {
                projectiles.remove(i);
                continue;
            }
            if !enemy.is_alive() || !enemy.is_in_screen() {
                continue;
            }
            if enemy.last_shot.is_none()
                || (enemy.last_shot.is_some()
                    && now - enemy.last_shot.unwrap() >= Duration::from_millis(enemy.shoot_freq))
            {
                let player_center_x =
                    player.shape.get_box().x + (player.shape.get_box().width as i32) / 2;
                let player_center_y =
                    player.shape.get_box().y + (player.shape.get_box().height as i32) / 2;
                projectiles.push(Projectile::from_sprite(
                    &enemy_shot,
                    &enemy.shape,
                    ENEMY_PROJECTILE_SPEED,
                    ProjShape::Circle,
                    LethalTo::Player,
                    Some((player_center_x, player_center_y)),
                ));
                enemy.last_shot = Some(now);
            }
            enemy.draw(&mut canvas)?;
            enemy.advance();
        }

        player.draw(&mut canvas)?;
        for proj in &mut projectiles {
            proj.draw(&mut canvas)?;
            proj.advance();
        }
        canvas.present();
        enemies.retain(|x| x.is_alive() && x.is_in_screen());
        projectiles.retain(|x| x.is_in_screen());
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 50));
    }
    Ok(())
}
