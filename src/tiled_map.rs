

use raylib::prelude::*;
use std::{collections::HashMap, error};
use rand::Rng;

pub struct TiledMap<'a>{
    tiles_textures: HashMap<i32, &'a Texture2D>,
    map: Vec<TiledMapLayer>,
    pub layers: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub scale: f32,
}

#[derive(Clone)]
pub struct TiledMapLayer {
    tiles: Vec<Vec<i32>>
}

impl TiledMapLayer {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        TiledMapLayer {
            tiles: vec![vec![0; size_y as usize]; size_x as usize]
        }
    }
}

impl<'a> TiledMap<'a> {
    pub fn new(layers: i32, size_x: i32, size_y: i32) -> Self {
        println!("TILEDMAP ATLAS CREATED");
        let mut tiled_map = TiledMap {
            tiles_textures: HashMap::new(),
            map: vec![TiledMapLayer::new(size_x, size_y); layers as usize],    
            layers,   
            size_x,
            size_y,
            tile_width: 32,
            tile_height: 32,
            scale: 2.0,
        };
            // Set every tile on layer 0 to have id 1
        for x in 0..tiled_map.size_x {
            for y in 0..tiled_map.size_y {
                tiled_map.set_tile(0, x, y, 1);
            }
        }
        // Randomize the id of a few tiles on layer 1
        let mut rng = rand::rng();
        for _ in 0..40 {
            let x = rng.random_range(0..tiled_map.size_x);
            let y = rng.random_range(0..tiled_map.size_y);
            let tile_id = rng.random_range(2..7 as i32);
            tiled_map.set_tile(1, x, y, tile_id);
        }
        tiled_map
    }

    pub fn add_tile_texture(&mut self, id: i32, texture: &'a Texture2D) {
        self.tiles_textures.insert(id, texture);
    }

    pub fn set_tile(&mut self, layer: i32, x: i32, y: i32, id: i32) {
        if x >= self.size_x || y >= self.size_y {
            return;
        }
        self.map[layer as usize].tiles[x as usize][y as usize] = id;
    }

    pub fn get_tile(&self, layer: i32, x: i32, y: i32) -> Option<&Texture2D> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        }
        let tile_id = self.map[layer as usize].tiles[x as usize][y as usize];
        return self.tiles_textures.get(&tile_id).map(|v| &**v);
    }

    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        for layer in 0..self.layers{
            for x in 0..self.size_x {
                for y in 0..self.size_y {
                    if let Some(texture) = self.get_tile(layer, x, y) {
                        //d.draw_texture(texture, x * self.tile_width, y * self.tile_height, Color::WHITE);
                        let position: Vector2 = Vector2 { 
                            x: (x * self.tile_width) as f32 * self.scale, 
                            y: (y * self.tile_height) as f32 * self.scale 
                        };
                        d.draw_texture_ex(texture, position, 0.0, self.scale, Color::WHITE);
                    }
                }
            }
        }
    }

}