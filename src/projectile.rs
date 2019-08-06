use crate::collide::Shape;
use crate::player::Player;
use crate::Velocity;
use crate::{HEIGHT, WIDTH};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const PLAYER_PROJECTILE_SPEED: i32 = 10;

pub struct Projectile<'a, 'b> {
    v: Velocity,
    damage: u32,
    sprite: &'a Texture<'b>,
    pub shape: Shape,
}

impl<'a, 'b> Projectile<'a, 'b> {
    pub fn from_sprite_rect(sprite: &'a Texture<'b>, player: &Player) -> Self {
        let x = player.x + player.width as i32;
        let y = player.y + (player.height / 2) as i32;
        Projectile {
            v: Velocity {
                x: PLAYER_PROJECTILE_SPEED,
                y: 0,
            },
            damage: 10,
            sprite,
            shape: Shape::new_rectangle(x, y, sprite.query().width, sprite.query().height),
        }
    }

    pub fn advance(&mut self) {
        self.shape.advance(&self.v);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let bounding_box = self.shape.get_box();
        canvas.copy(
            self.sprite,
            None,
            Rect::new(
                bounding_box.x,
                bounding_box.y,
                bounding_box.width,
                bounding_box.height,
            ),
        )
    }

    pub fn is_in_screen(&self) -> bool {
        self.shape.is_in_screen()
    }
}
