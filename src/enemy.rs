use crate::collide::Shape;
use crate::Projectile;
use crate::{Axis, Velocity};
use crate::{HEIGHT, WIDTH};
use rand::prelude::*;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Enemy<'a, 'b> {
    sprite: &'a Texture<'b>,
    v: Velocity,
    alive: bool,
    pub shape: Shape,
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
            v: Velocity::new(-rng.gen_range(MIN_SPEED, MAX_SPEED + 1), 0),
            alive: true,
            shape: Shape::new_rectangle(
                (WIDTH - w) as i32,
                rng.gen_range(1, HEIGHT - h) as i32,
                h,
                w,
            ),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let bounding_box = self.shape.get_box();
        let rect = Rect::new(
            bounding_box.x,
            bounding_box.y,
            bounding_box.width,
            bounding_box.height,
        );
        canvas.copy(self.sprite, None, rect)
    }

    pub fn is_in_screen(&self) -> bool {
        self.shape.is_in_screen()
    }

    pub fn advance(&mut self) {
        self.shape.advance(&self.v)
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}
