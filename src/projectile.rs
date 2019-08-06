use crate::collide;
use crate::collide::Shape;
use crate::player::Player;
use crate::Velocity;
use crate::{HEIGHT, WIDTH};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Projectile<'a, 'b> {
    v: Velocity,
    damage: u32,
    sprite: &'a Texture<'b>,
    pub shape: Shape,
    lethal_to: LethalTo,
}

pub enum ProjShape {
    Rectangle,
    Circle,
}

#[derive(Eq, PartialEq, Debug)]
pub enum LethalTo {
    Player,
    Enemy,
}

// Get projectile velocity vector which:
// * points from coords1 to coords2
// * has length of magnitude
fn proj_velocity(coords0: (i32, i32), coords1: (i32, i32), magnitude: f64) -> Velocity {
    let delta_x: f64 = (coords1.0 - coords0.0) as f64;
    let delta_y: f64 = (coords1.1 - coords0.1) as f64;
    let hypotenuse = f64::sqrt((delta_x * delta_x) + (delta_y * delta_y));
    let mut unit_x: f64 = 0.0;
    let mut unit_y: f64 = 0.0;
    if hypotenuse != 0.0 {
        unit_x = delta_x / hypotenuse;
        unit_y = delta_y / hypotenuse;
    }
    Velocity {
        x: unit_x * magnitude,
        y: unit_y * magnitude,
    }
}

impl<'a, 'b> Projectile<'a, 'b> {
    pub fn from_sprite(
        sprite: &'a Texture<'b>,
        ship: &Shape,
        speed: f64,
        proj_shape: ProjShape,
        lethal_to: LethalTo,
        target: Option<(i32, i32)>,
    ) -> Self {
        let bounding_box = ship.get_box();
        let (x, y) = match ship {
            Shape::Rectangle(collide::Rectangle { x, y, .. })
            | Shape::Point(collide::Point { x, y }) => {
                if speed > 0.0 {
                    (
                        x + bounding_box.width as i32,
                        y + (bounding_box.height / 2) as i32,
                    )
                } else {
                    (*x, y + (bounding_box.height / 2) as i32)
                }
            }
            Shape::Circle(collide::Circle { x, y, r }) => {
                if speed > 0.0 {
                    (
                        x + *r as i32,
                        y + (bounding_box.height / 2) as i32 - *r as i32,
                    )
                } else {
                    (
                        x - *r as i32,
                        y - (bounding_box.height / 2) as i32 - *r as i32,
                    )
                }
            }
            _ => {
                if speed > 0.0 {
                    (
                        bounding_box.x + bounding_box.width as i32,
                        bounding_box.y + (bounding_box.height / 2) as i32,
                    )
                } else {
                    (
                        bounding_box.x,
                        bounding_box.y - (bounding_box.height / 2) as i32,
                    )
                }
            }
        };
        Projectile {
            v: match target {
                None => Velocity { x: speed, y: 0.0 },
                Some(coords) => proj_velocity((x, y), (coords.0, coords.1), speed),
            },
            damage: 10,
            sprite,
            shape: match proj_shape {
                ProjShape::Rectangle => {
                    Shape::new_rectangle(x, y, sprite.query().width, sprite.query().height)
                }
                ProjShape::Circle => Shape::new_circle(x, y, sprite.query().width / 2),
            },
            lethal_to,
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

    pub fn lethal_to_enemy(&self) -> bool {
        self.lethal_to == LethalTo::Enemy
    }

    pub fn lethal_to_player(&self) -> bool {
        self.lethal_to == LethalTo::Player
    }
}
