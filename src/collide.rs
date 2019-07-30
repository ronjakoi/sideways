use crate::{enemy, player};

pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}
pub trait Rectangle {
    fn rect(&self) -> Rect;
}

pub trait Collider {
    fn collide<T: Rectangle>(&self, target: &T) -> bool;
}

impl Collider for enemy::Enemy<'_, '_> {
    fn collide<T: Rectangle>(&self, target: &T) -> bool {
        let rect1 = self.rect();
        let rect2 = target.rect();

        rect1.x < rect2.x + rect2.width as i32
            && rect1.x + rect1.width as i32 > rect2.x
            && rect1.y < rect2.y + rect2.height as i32
            && rect1.y + rect1.height as i32 > rect2.y
    }
}
