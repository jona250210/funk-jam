// basic item with position and borrowed texture
use raylib::prelude::*;

pub struct Item<'a> {
    pub position: Vector2,
    pub texture: &'a Texture2D,
    width: f32,
    height: f32,
    scale: f32,
}

impl<'a> Item<'a> {
    pub fn new(position: Vector2, texture: &'a Texture2D, scale: f32) -> Self {
        Item { 
            position, 
            texture,
            width: texture.width as f32 * scale,
            height: texture.height as f32 * scale,
            scale: scale,
        }
    }

    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        d.draw_texture_ex(self.texture, self.position, 0.0, self.scale, Color::WHITE);
    }
}