use crate::collide;
use crate::Velocity;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Player<'a, 'b> {
    sprite: &'a Texture<'b>,
    pub v: Velocity,
    pub shape: collide::Shape,
}

impl<'a, 'b> Player<'a, 'b> {
    pub fn from_sprite(sprite: &'a Texture<'b>) -> Self {
        Player {
            sprite,
            v: Velocity::new(0.0, 0.0),
            shape: collide::Shape::new_rectangle(
                crate::WIDTH as i32 / 5,
                crate::HEIGHT as i32 / 2,
                sprite.query().width,
                sprite.query().height,
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

    pub fn apply_velocity(&mut self) {
        match &mut self.shape {
            collide::Shape::Rectangle(collide::Rectangle {
                x,
                y,
                width,
                height,
                ..
            }) => {
                let max_x = (crate::WIDTH - 1 - *width) as i32 - self.v.x as i32;
                let max_y = (crate::HEIGHT - 1 - *height) as i32 - self.v.y as i32;
                if *x >= 0 && *x < max_x {
                    *x += self.v.x as i32;
                } else if *x < 0 {
                    *x = 0;
                    self.v = Velocity::new(0.0, self.v.y);
                } else if *x >= max_x {
                    *x = max_x - 1;
                    self.v = Velocity::new(0.0, self.v.y);
                }

                if *y >= 0 && *y < max_y {
                    *y += self.v.y as i32;
                } else if *y < 0 {
                    *y = 0;
                    self.v = Velocity::new(self.v.x, 0.0);
                } else if *y >= max_y {
                    *y = max_y - 1;
                    self.v = Velocity::new(self.v.x, 0.0);
                }
            }
            _ => {} // TODO Compound
        }
    }
}
