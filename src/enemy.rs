use crate::collide;
use crate::Projectile;
use crate::{Axis, Velocity};
use crate::{HEIGHT, WIDTH};
use rand::prelude::*;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Enemy<'a, 'b> {
    x: i32,
    y: i32,
    sprite: &'a Texture<'b>,
    width: u32,
    height: u32,
    v: Velocity,
    alive: bool,
}

impl collide::Rectangle for Enemy<'_, '_> {
    fn rect(&self) -> collide::Rect {
        collide::Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl<'a, 'b> Enemy<'a, 'b> {
    pub fn from_sprite(sprite: &'a Texture<'b>) -> Self {
        const MAX_SPEED: i32 = 4;
        const MIN_SPEED: i32 = 1;
        let mut rng = thread_rng();
        let h = sprite.query().height;
        let w = sprite.query().width;

        Enemy {
            sprite,
            width: w,
            height: h,
            x: (WIDTH - w) as i32,
            y: rng.gen_range(1, HEIGHT - h) as i32,
            v: Velocity::new(-rng.gen_range(MIN_SPEED, MAX_SPEED + 1), 0),
            alive: true,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        canvas.copy(self.sprite, None, rect)
    }

    pub fn is_in_screen(&self) -> bool {
        self.x >= -(self.width as i32)
            && self.x <= WIDTH as i32
            && self.y >= -(self.height as i32)
            && self.y <= HEIGHT as i32
    }

    pub fn advance(&mut self) {
        self.x += self.v.x;
        self.y += self.v.y;
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

}
