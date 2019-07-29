use crate::Velocity;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Player<'a, 'b> {
    sprite: &'a Texture<'b>,
    pub x: i32,
    pub y: i32,
    pub v: Velocity,
    pub width: u32,
    pub height: u32,
}

impl<'a, 'b> Player<'a, 'b> {
    pub fn from_sprite(sprite: &'a Texture<'b>) -> Self {
        Player {
            sprite,
            x: crate::WIDTH as i32 / 5,
            y: crate::HEIGHT as i32 / 2,
            v: Velocity::new(0, 0),
            width: sprite.query().width,
            height: sprite.query().height,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        canvas.copy(self.sprite, None, rect)
    }

    pub fn apply_velocity(&mut self) {
        let max_x = (crate::WIDTH - 1 - self.width) as i32 - self.v.x;
        let max_y = (crate::HEIGHT - 1 - self.height) as i32 - self.v.y;

        if self.x >= 0 && self.x < max_x {
            self.x += self.v.x;
        } else if self.x < 0 {
            self.x = 0;
            self.v = Velocity::new(0, self.v.y);
        } else if self.x >= max_x {
            self.x = max_x - 1;
            self.v = Velocity::new(0, self.v.y);
        }

        if self.y >= 0 && self.y < max_y {
            self.y += self.v.y;
        } else if self.y < 0 {
            self.y = 0;
            self.v = Velocity::new(self.v.x, 0);
        } else if self.y >= max_y {
            self.y = max_y - 1;
            self.v = Velocity::new(self.v.x, 0);
        }
    }
}
