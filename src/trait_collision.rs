use raylib::prelude::Rectangle;

pub trait Collision {
    fn collision_with_rec(&self, other: &Rectangle) -> bool;
}
