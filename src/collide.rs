use crate::Velocity;
use crate::{enemy, player, projectile};
use crate::{HEIGHT, WIDTH};

#[derive(Eq, PartialEq, Debug)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Rectangle(Rectangle),
    Compound(Vec<Shape>),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
#[derive(Eq, PartialEq, Debug)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub r: u32,
}

impl Shape {
    pub fn new_point(x: i32, y: i32) -> Self {
        Shape::Point(Point { x, y })
    }

    pub fn new_circle(x: i32, y: i32, r: u32) -> Self {
        Shape::Circle(Circle { x, y, r })
    }

    pub fn new_rectangle(x: i32, y: i32, width: u32, height: u32) -> Self {
        Shape::Rectangle(Rectangle {
            x,
            y,
            width,
            height,
        })
    }

    pub fn new_compound() -> Self {
        Shape::Compound(vec![])
    }

    pub fn push(&mut self, item: Shape) -> Result<(), String> {
        match self {
            Shape::Compound(shapes) => {
                shapes.push(item);
                Ok(())
            }
            _ => Err("Not a compound shape".to_string()),
        }
    }

    // get bounding box
    pub fn get_box(&self) -> Rectangle {
        match self {
            Shape::Rectangle(rect) => *rect,
            Shape::Circle(c) => {
                let box_x = c.x - c.r as i32;
                let box_y = c.y - c.r as i32;
                let box_w = c.r * 2;
                let box_h = box_w;
                Rectangle {
                    x: box_x,
                    y: box_y,
                    width: box_w as u32,
                    height: box_h as u32,
                }
            }
            Shape::Point(p) => Rectangle {
                x: p.x,
                y: p.y,
                width: 0,
                height: 0,
            },
            _ => Rectangle {
                // TODO
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        }
    }

    pub fn advance(&mut self, v: &Velocity) {
        match self {
            Shape::Compound(c) => {
                for s in c.iter_mut() {
                    match s {
                        _ => {}
                    } //TODO
                }
            }
            Shape::Point(Point { x, y })
            | Shape::Circle(Circle { x, y, .. })
            | Shape::Rectangle(Rectangle { x, y, .. }) => {
                *x += v.x.ceil() as i32;
                *y += v.y.ceil() as i32;
            }
        }
    }

    pub fn is_in_screen(&self) -> bool {
        let bounding_box = self.get_box();
        bounding_box.x >= -(bounding_box.width as i32)
            && bounding_box.x <= WIDTH as i32
            && bounding_box.y >= -(bounding_box.height as i32)
            && bounding_box.y <= HEIGHT as i32
    }
}

pub trait Collider {
    fn collide(&self, target: &Shape) -> bool;
    fn is_in_screen(&self) -> bool;
}

impl Collider for Shape {
    fn collide(&self, target: &Shape) -> bool {
        match self {
            Shape::Rectangle(me) => match target {
                Shape::Rectangle(other) => collide_shapes::collide_rect_to_rect(me, other),
                Shape::Point(other) => collide_shapes::collide_rect_to_point(me, other),
                Shape::Circle(other) => collide_shapes::collide_rect_to_circle(me, other),
                Shape::Compound(shapes) => shapes.iter().any(|x| x.collide(self)),
            },
            Shape::Circle(me) => match target {
                Shape::Rectangle(other) => collide_shapes::collide_rect_to_circle(other, me),
                Shape::Point(other) => collide_shapes::collide_circle_to_point(me, other),
                Shape::Circle(other) => collide_shapes::collide_circle_to_circle(me, other),
                Shape::Compound(shapes) => shapes.iter().any(|x| x.collide(self)),
            },
            Shape::Point(me) => match target {
                Shape::Rectangle(other) => collide_shapes::collide_rect_to_point(other, me),
                Shape::Point(other) => me == other,
                Shape::Circle(other) => collide_shapes::collide_circle_to_point(other, me),
                Shape::Compound(shapes) => shapes.iter().any(|x| x.collide(self)),
            },
            Shape::Compound(shapes) => shapes.iter().any(|x| x.collide(target)),
        }
    }

    fn is_in_screen(&self) -> bool {
        match self {
            Shape::Rectangle(rect) => {
                rect.x >= -(rect.width as i32)
                    && rect.x <= WIDTH as i32
                    && rect.y >= -(rect.height as i32)
                    && rect.y <= HEIGHT as i32
            }
            Shape::Point(point) => {
                point.x >= 0 && point.x < WIDTH as i32 && point.y >= 0 && point.y < HEIGHT as i32
            }
            Shape::Circle(circle) => {
                circle.x + circle.r as i32 >= 0
                    && circle.x - (circle.r as i32) < WIDTH as i32
                    && circle.y + circle.r as i32 >= 0
                    && circle.y - (circle.r as i32) < HEIGHT as i32
            }
            Shape::Compound(shapes) => shapes.iter().any(|x| x.is_in_screen()),
        }
    }
}

impl Collider for enemy::Enemy<'_, '_> {
    fn collide(&self, other: &Shape) -> bool {
        self.shape.collide(other)
    }

    fn is_in_screen(&self) -> bool {
        self.shape.is_in_screen()
    }
}

impl Collider for projectile::Projectile<'_, '_> {
    fn collide(&self, other: &Shape) -> bool {
        self.shape.collide(other)
    }

    fn is_in_screen(&self) -> bool {
        self.shape.is_in_screen()
    }
}

mod collide_shapes {
    use super::{Circle, Point, Rectangle};
    use super::{Collider, Shape};

    pub fn collide_rect_to_rect(rect1: &Rectangle, rect2: &Rectangle) -> bool {
        rect1.x < rect2.x + rect2.width as i32
            && rect1.x + rect1.width as i32 > rect2.x
            && rect1.y < rect2.y + rect2.height as i32
            && rect1.y + rect1.height as i32 > rect2.y
    }

    pub fn collide_rect_to_point(rect: &Rectangle, point: &Point) -> bool {
        point.x >= rect.x
            && point.x <= point.x + rect.width as i32
            && point.y >= rect.y
            && point.y <= point.y + rect.height as i32
    }

    pub fn collide_rect_to_circle(rect: &Rectangle, circle: &Circle) -> bool {
        let test_x = {
            if circle.x < rect.x {
                rect.x
            } else {
                rect.x + rect.width as i32
            }
        };

        let test_y = if circle.y < rect.y {
            rect.y
        } else {
            rect.y + rect.height as i32
        };

        let dist_x = circle.x - test_x;
        let dist_y = circle.y - test_y;

        let distance = (((dist_x * dist_x) + (dist_y * dist_y)) as f64).sqrt();

        if distance <= circle.r as f64 {
            true
        } else {
            false
        }
    }

    pub fn collide_circle_to_circle(circle1: &Circle, circle2: &Circle) -> bool {
        let dist_x = circle1.x + circle1.r as i32 - circle2.x + circle2.r as i32;
        let dist_y = circle1.y + circle1.r as i32 - circle2.y + circle2.r as i32;

        let distance = (((dist_x * dist_x) + (dist_y * dist_y)) as f64).sqrt();

        if distance <= circle1.r as f64 || distance <= circle2.r as f64 {
            true
        } else {
            false
        }
    }

    pub fn collide_circle_to_point(circle: &Circle, point: &Point) -> bool {
        let dist_x = circle.x - point.x;
        let dist_y = circle.y - point.y;

        let distance = (((dist_x * dist_x) + (dist_y * dist_y)) as f64).sqrt();

        if distance <= circle.r as f64 {
            true
        } else {
            false
        }
    }
}
