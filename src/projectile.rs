use crate::collide;
use crate::Velocity;
use crate::{HEIGHT, WIDTH};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Projectile<'a, 'b> {
    pub v: Velocity,
    pub x: i32,
    pub y: i32,
    pub damage: u32,
    pub sprite: &'a Texture<'b>,
    pub width: u32,
    pub height: u32,
}

impl collide::Rectangle for Projectile<'_, '_> {
    fn rect(&self) -> collide::Rect {
        collide::Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl<'a, 'b> Projectile<'a, 'b> {
    pub fn advance(&mut self) {
        self.x += self.v.x;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(
            self.sprite,
            None,
            Rect::new(self.x, self.y, self.width, self.height),
        )
    }

    pub fn is_in_screen(&self) -> bool {
        self.x >= -(self.width as i32)
            && self.x <= WIDTH as i32
            && self.y >= -(self.height as i32)
            && self.y <= HEIGHT as i32
    }
}
