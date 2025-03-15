use raylib::prelude::*;

pub trait Collision {
    fn collision_with_rec(&self) -> bool;
}
