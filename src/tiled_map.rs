use raylib::prelude::*;
use std::{collections::HashMap, error};
use rand::Rng;

use crate::texture_atlas::TextureAtlas;

const TILE_WIDTH: i32 = 32;
const TILE_HEIGHT: i32 = 32;
const SCALE: f32 = 2.0;

pub struct TiledMap<'a> {
    tiles_textures: HashMap<i32, &'a Texture2D>,
    map: Vec<TiledMapLayer>,
    pub layers: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub scale: f32,
    pub tiles_textures_paths: Vec<&'a str>,
    animation_loops: Vec<Vec<i32>>,
    one_time_animations: Vec<Vec<i32>>,
    animation_counter: f32,
    animated_tiles: Vec<(i32, i32, i32)>,
}

#[derive(Clone)]
pub struct TiledMapLayer {
    tiles: Vec<Vec<i32>>,
}

impl TiledMapLayer {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        TiledMapLayer {
            tiles: vec![vec![0; size_y as usize]; size_x as usize],
        }
    }
}

impl<'a> TiledMap<'a> {
    pub fn new(layers: i32, size_x: i32, size_y: i32, atlas: &'a TextureAtlas) -> Self {
        println!("TILEDMAP ATLAS CREATED");
        let mut tiled_map = TiledMap {
            tiles_textures: HashMap::new(),
            map: vec![TiledMapLayer::new(size_x, size_y); layers as usize],
            layers,
            size_x,
            size_y,
            tile_width: TILE_WIDTH,
            tile_height: TILE_HEIGHT,
            scale: SCALE,
            tiles_textures_paths: vec![
                /* 0*/ "assets/empty_tile.png",
                /* 1*/ "assets/sand_tile.png",
                /* 2*/ "assets/palme0.png",
                /* 3*/ "assets/palme1.png",
                /* 4*/ "assets/palme2.png",
                /* 5*/ "assets/palme3.png",
                /* 6*/ "assets/empty_tile.png",
                /* 7*/ "assets/stein0.png",
                /* 8*/ "assets/stein1.png",
                /* 9*/ "assets/stein2.png",
                /*10*/ "assets/stein3.png",
                /*11*/ "assets/stein4.png",
                /*12*/ "assets/empty_tile.png",
            ],
            animation_loops: vec![
                //vec![2, 3, 4, 5],
            ],
            one_time_animations: vec![
                vec![2, 3, 4, 5, 6],
                vec![7, 8, 9, 10, 12],
            ],
            animation_counter: 0.0,
            animated_tiles: Vec::new(),
        };
        tiled_map.load_textures(atlas);
        tiled_map.initialize_tiles();
        tiled_map.randomize_tiles();

        tiled_map
    }

    fn load_textures(&mut self, atlas: &'a TextureAtlas) {
        let texture_paths: Vec<_> = self.tiles_textures_paths.iter().enumerate().map(|(id, &path)| (id as i32, path)).collect();
        for (id, path) in texture_paths {
            let texture = atlas.get_texture(path);
            self.add_tile_texture(id, texture);
        }
    }

    fn initialize_tiles(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                self.set_tile(0, x, y, 1, false);
            }
        }
    }

    fn randomize_tiles(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..40 {
            let x = rng.random_range(0..self.size_x);
            let y = rng.random_range(0..self.size_y);
            let tile_id = rng.random_range(2..7);
            self.set_tile(1, x, y, tile_id, true);
        }
        self.set_tile(1, 1, 1, 2, false);
        self.set_tile(1, 2, 1, 7, false);
    }

    pub fn add_tile_texture(&mut self, id: i32, texture: &'a Texture2D) {
        self.tiles_textures.insert(id, texture);
    }

    pub fn set_tile(&mut self, layer: i32, x: i32, y: i32, id: i32, animated: bool) {
        if x >= self.size_x || y >= self.size_y {
            return;
        }
        if animated {
            if !self.animated_tiles.contains(&(layer, x, y)) {
                self.animated_tiles.push((layer, x, y));
            }
        } else {
            self.animated_tiles.retain(|&v| v != (layer, x, y));
        }
        self.map[layer as usize].tiles[x as usize][y as usize] = id;
    }

    pub fn get_tile_texture(&self, layer: i32, x: i32, y: i32) -> Option<&Texture2D> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        }
        let tile_id = self.map[layer as usize].tiles[x as usize][y as usize];
        self.tiles_textures.get(&tile_id).map(|v| &**v)
    }

    pub fn get_tile_id(&self, layer: i32, x: i32, y: i32) -> i32 {
        if x >= self.size_x || y >= self.size_y {
            return -1;
        }
        self.map[layer as usize].tiles[x as usize][y as usize]
    }

    pub fn update_animated_tiles(&mut self, delta_time: f32) {
        self.animation_counter += delta_time;
        if self.animation_counter < 1.0 {
            return;
        }
        self.animation_counter = 0.0;
        let mut updates = Vec::new();
        for (layer, x, y) in &self.animated_tiles {
            let tile_id = self.get_tile_id(*layer, *x, *y);
            for animation_loop in &self.animation_loops {
                if let Some(pos) = animation_loop.iter().position(|&r| r == tile_id) {
                    let new_tile_id = animation_loop[(pos + 1) % animation_loop.len()];
                    updates.push((*layer, *x, *y, new_tile_id));
                }
            }
            for one_time_animation in &self.one_time_animations {
                if let Some(pos) = one_time_animation.iter().position(|&r| r == tile_id) {
                    if pos + 1 < one_time_animation.len() {
                        let new_tile_id = one_time_animation[pos + 1];
                        updates.push((*layer, *x, *y, new_tile_id));
                    }
                }
            }
        }

        for (layer, x, y, new_tile_id) in updates {
            self.set_tile(layer, x, y, new_tile_id, true);
        }
    }

    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        for layer in 0..self.layers {
            for x in 0..self.size_x {
                for y in 0..self.size_y {
                    if let Some(texture) = self.get_tile_texture(layer, x, y) {
                        let position = Vector2 {
                            x: (x * self.tile_width) as f32 * self.scale,
                            y: (y * self.tile_height) as f32 * self.scale,
                        };
                        d.draw_texture_ex(texture, position, 0.0, self.scale, Color::WHITE);
                    }
                }
            }
        }
    }
}