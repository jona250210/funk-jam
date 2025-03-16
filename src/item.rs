// basic item with position and borrowed texture
use crate::trait_collision::Collision;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Item<'a> {
    pub position: Vector2,
    pub texture: &'a Texture2D,
    width: f32,
    height: f32,
    scale: f32,
    pub item_type: ItemType,
}

#[derive(Clone)]
pub enum ItemType {
    Axe,
    Pickaxe,
    Gear,
    Shovel,
}

impl<'a> Item<'a> {
    pub fn new(position: Vector2, texture: &'a Texture2D, scale: f32, item_type: ItemType) -> Self {
        Item {
            position,
            texture,
            width: texture.width as f32 * scale,
            height: texture.height as f32 * scale,
            scale,
            item_type,
        }
    }

    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        d.draw_texture_ex(self.texture, self.position, 0.0, self.scale, Color::WHITE);
    }
}

impl Collision for Item<'_> {
    fn collision_with_rec(&self, other: &Rectangle) -> bool {
        Rectangle::new(
            self.position.x as f32,
            self.position.y as f32,
            self.width,
            self.height,
        )
        .check_collision_recs(other)
    }
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
