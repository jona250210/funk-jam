use rand::Rng;
use raylib::prelude::*;
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, io::Read, path::Path};

use crate::texture_atlas::TextureAtlas;
use crate::trait_collision::Collision;

pub const TILE_WIDTH: i32 = 32;
pub const TILE_HEIGHT: i32 = 32;
pub const SCALE: f32 = 2.0;

pub struct TiledMap<'a> {
    tiles_textures: HashMap<i32, &'a Texture2D>,
    map: Vec<TiledMapLayer>,
    pub layers: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub shift_x: i32,
    pub shift_y: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub scale: f32,
    pub tiles_textures_paths: Vec<&'a str>,
    animation_counter: f32,
}

type TextureID = i32;

#[derive(Clone, PartialEq, Eq)]
pub enum Tags {
    Barrier,
    Destroyable,
}

#[derive(Clone)]
pub enum Tile {
    Static(TextureID, Vec<Tags>),
    Animated(Vec<TextureID>, usize, Vec<Tags>),
    AnimatedOnce(Vec<TextureID>, usize, Vec<Tags>),
}

pub fn new_stone() -> Tile {
    Tile::Static(7, vec![Tags::Barrier, Tags::Destroyable])
}

pub fn new_palme() -> Tile {
    Tile::Static(2, vec![Tags::Barrier, Tags::Destroyable])
}

pub fn new_pile() -> Tile {
    Tile::Static(18, vec![Tags::Barrier, Tags::Destroyable])
}

#[derive(Clone)]
pub struct TiledMapLayer {
    tiles: Vec<Vec<Tile>>,
}

impl TiledMapLayer {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        TiledMapLayer {
            tiles: vec![vec![Tile::Static(0, Vec::new()); size_y as usize]; size_x as usize],
        }
    }

    pub fn get_collision_tiles(& self, other: &Rectangle) -> Option<Vec<(& Tile, Vector2)>> {
        // TODO: Effizienter machen
        let mut collisions: Vec<(& Tile, Vector2)> = Vec::new();
        let width;
        let height;

        {
            width = self.tiles.len();
            height = self.tiles[0].len();
        }

        for x in 0..width {
            for y in 0..height {
                let tmp = Rectangle::new(
                    (x as i32 * TILE_WIDTH) as f32 * SCALE,
                    (y as i32 * TILE_HEIGHT) as f32 * SCALE,
                    TILE_WIDTH as f32 * SCALE,
                    TILE_HEIGHT as f32 * SCALE,
                );
                if tmp.check_collision_recs(other) {
                    let lol = & self.tiles[x][y];
                    collisions.push((
                        lol,
                        Vector2::new(
                            (x as i32 * TILE_WIDTH) as f32 * SCALE,
                            (y as i32 * TILE_HEIGHT) as f32 * SCALE,
                        ),
                    ));
                }
            }
        }

        if collisions.is_empty() {
            return None;
        }
        return Some(collisions);
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
            shift_x: 0,
            shift_y: 0,
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
                /*13*/ "assets/water0.png",
                /*14*/ "assets/water1.png",
                /*15*/ "assets/water2.png",
                /*16*/ "assets/water3.png",
                /*17*/ "assets/Sandmauer.png",
                /*18*/ "assets/pile0.png",
                /*19*/ "assets/pile1.png",
                /*20*/ "assets/pile2.png",
                /*21*/ "assets/pile3.png",
                /*22*/ "assets/pile4.png",
                /*23*/ "assets/pile5.png",
                /*24*/ "assets/empty_tile.png",
            ],
            animation_counter: 0.0,
        };
        tiled_map.load_textures(atlas);
        tiled_map.initialize_tiles();
        tiled_map.randomize_tiles();

        tiled_map
    }

    pub fn water(layers: i32, size_x: i32, size_y: i32, atlas: &'a TextureAtlas) -> Self {
        let mut tiled_map = TiledMap {
            tiles_textures: HashMap::new(),
            map: vec![TiledMapLayer::new(size_x, size_y); layers as usize],
            layers,
            size_x,
            size_y,
            shift_x: -320,
            shift_y: -320,
            tile_width: TILE_WIDTH,
            tile_height: TILE_HEIGHT,
            scale: SCALE,
            tiles_textures_paths: vec![
                /* 0*/ "assets/water0.png",
                /* 1*/ "assets/water1.png",
                /* 2*/ "assets/water2.png",
                /* 3*/ "assets/water3.png",
            ],
            animation_counter: 0.0,
        };
        tiled_map.load_textures(atlas);
        tiled_map.init_water();
        tiled_map
    }
    
    pub fn from(config: &MazeConfig, atlas: &'a TextureAtlas) -> Result<Self, String> {
        let mut tiled_map = TiledMap::new(2, config.size.0, config.size.1, atlas);
        let mut ground_iter = config.ground.chars().filter(|c| c != &'\n' && c != &' ' && c != &'\r');
        let mut objects_iter = config.objects.chars().filter(|c| c != &'\n' && c != &' ' && c != &'\r');

        // ground
        for y in 0..tiled_map.size_y {
            for x in 0..tiled_map.size_x {
                tiled_map.set_tile(
                    0,
                    x,
                    y,
                    match ground_iter.next() {
                        Some('0') => Tile::Static(0, Vec::new()),
                        Some('1') => Tile::Animated(vec![13, 14, 15, 16], 0, vec![Tags::Barrier]),
                        Some('2') => Tile::Static(1, Vec::new()),
                        Some('3') => Tile::Static(17, vec![Tags::Barrier]),
                        Some('4') => new_palme(),
                        Some('5') => new_stone(),
                        Some('6') => new_pile(),

                        Some(c) => return Err(format!("MazeConfig id {} is invalid", c)),
                        None => return Err("MazeConfig ground is too short somehow".to_string()),
                    },
                )
            }
        }

        // objects
        for y in 0..tiled_map.size_y {
            for x in 0..tiled_map.size_x {
                tiled_map.set_tile(
                    1,
                    x,
                    y,
                    match objects_iter.next() {
                        Some('0') => Tile::Static(0, Vec::new()),
                        Some('1') => Tile::Animated(vec![13, 14, 15, 16], 0, vec![Tags::Barrier]),
                        Some('2') => Tile::Static(1, Vec::new()),
                        Some('3') => Tile::Static(17, vec![Tags::Barrier]),
                        Some('4') => new_palme(),
                        Some('5') => new_stone(),
                        Some('6') => new_pile(),

                        Some(c) => return Err(format!("MazeConfig id {} is invalid", c)),
                        None => return Err("MazeConfig object is too short somehow".to_string()),
                    },
                )
            }
        }

        return Ok(tiled_map);
    }

    fn load_textures(&mut self, atlas: &'a TextureAtlas) {
        let texture_paths: Vec<_> = self
            .tiles_textures_paths
            .iter()
            .enumerate()
            .map(|(id, &path)| (id as i32, path))
            .collect();
        for (id, path) in texture_paths {
            let texture = atlas.get_texture(path);
            self.add_tile_texture(id, texture);
        }
    }

    pub fn handle_hit_tiles(&mut self, marked_tiles: Vec<(Tile, Vector2)>){
        for (tile, pos) in marked_tiles {
            match tile {
                Tile::Static(id, tags) => {
                    let x = (pos.x as i32 / (TILE_WIDTH as i32 * SCALE as i32)) as usize;
                    let y = (pos.y as i32 / (TILE_HEIGHT as i32 * SCALE as i32)) as usize;
                    println!("{} {}", x, y);

                    match id {
                        2 => {
                            self.map[1 as usize].tiles[x][y] = Tile::AnimatedOnce(vec![2, 3, 4, 5, 6], 0, vec![Tags::Barrier]);
                        }
                        7 => {
                            self.map[1 as usize].tiles[x][y] = Tile::AnimatedOnce(vec![7, 8, 9, 10, 11, 12], 0, vec![Tags::Barrier]);
                        }
                        18 => {
                            self.map[1 as usize].tiles[x][y] = Tile::AnimatedOnce(vec![18, 19, 20, 21, 22, 23, 24], 0, vec![Tags::Barrier]);
                        }
                        _ => ()
                    }
                    
                },
                Tile::Animated(_, _, _) => (),
                Tile::AnimatedOnce(_, _, _) => (),
            }
        }

        for layer in 0..self.layers {
            for x in 0..self.size_x {
                for y in 0..self.size_y {
                    
                }
            }
        }
    }


    fn initialize_tiles(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if x > 3 && x < 15 && y > 3 && y < 15 {
                    self.set_tile(0, x, y, Tile::Static(1, Vec::new()))
                } else {
                    self.set_tile(
                        0,
                        x,
                        y,
                        Tile::Animated(vec![13, 14, 15, 16], 0, vec![Tags::Barrier]),
                    );
                }
            }
        }
    }

    fn init_water(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {                
                self.set_tile(
                    0,
                    x,
                    y,
                    Tile::Animated(vec![0, 1, 2, 3], 0, vec![Tags::Barrier]),
                );
            }
        }
    }

    fn randomize_tiles(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..40 {
            let x = rng.random_range(0..self.size_x);
            let y = rng.random_range(0..self.size_y);

            let tile_id = rng.random_range(0..2);
            let status = rng.random_range(0..3);

            //let tile = match (tile_id, status) {
            //    (0, 0) => Tile::Static(2, vec![Tags::Barrier, Tags::Destroyable]),
            //    (0, 1) => Tile::Animated(vec![2, 3, 4, 5], 0, vec![Tags::Barrier]),
            //    (0, 2) => Tile::AnimatedOnce(vec![2, 3, 4, 5, 6], 0, vec![Tags::Barrier]),
            //
            //    (1, 0) => Tile::Static(7, vec![Tags::Barrier, Tags::Destroyable]),
            //    (1, 1) => Tile::Animated(vec![7, 8, 9, 10, 11], 0, vec![Tags::Barrier]),
            //    (1, 2) => Tile::AnimatedOnce(vec![7, 8, 9, 10, 11, 12], 0, vec![Tags::Barrier]),
            //    _ => Tile::Static(0, Vec::new()),
            //};

            let tile = match tile_id {
                0 => Tile::Static(2, vec![Tags::Barrier, Tags::Destroyable]),
                1 => Tile::Static(7, vec![Tags::Barrier, Tags::Destroyable]),
                _ => Tile::Static(0, Vec::new()),
            };

            self.set_tile(1, x, y, tile);
        }
    }

    pub fn add_tile_texture(&mut self, id: i32, texture: &'a Texture2D) {
        self.tiles_textures.insert(id, texture);
    }

    pub fn set_tile(&mut self, layer: i32, x: i32, y: i32, tile: Tile) {
        if x >= self.size_x || y >= self.size_y {
            return;
        }
        self.map[layer as usize].tiles[x as usize][y as usize] = tile;
    }

    pub fn get_tile_texture(&self, layer: i32, x: i32, y: i32) -> Option<&Texture2D> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        }
        let tile = &self.map[layer as usize].tiles[x as usize][y as usize];
        self.tiles_textures
            .get(&match tile {
                Tile::Static(id, _) => *id,
                Tile::Animated(items, current, _) => items[*current] as i32,
                Tile::AnimatedOnce(items, current, _) => items[*current] as i32,
            })
            .map(|v| *v)
    }

    pub fn get_tile_id(&self, layer: i32, x: i32, y: i32) -> Option<i32> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        }
    
        match self.map[layer as usize].tiles[x as usize][y as usize] {
            Tile::Static(id, _) => Some(id),
            Tile::Animated(_, current, _) => Some(current as i32),
            Tile::AnimatedOnce(_, current, _) => Some(current as i32),
        }
    }

    pub fn update_animated_tiles(&mut self, delta_time: f32) {
        self.animation_counter += delta_time;
        if self.animation_counter < 0.3 {
            return;
        }

        self.animation_counter = 0.0;

        for (z, l) in &mut self.map.iter_mut().enumerate() {
            for (x, row) in l.tiles.iter_mut().enumerate() {
                for (y, tile) in row.iter_mut().enumerate() {
                    match tile {
                        Tile::Static(_, _) => (),
                        Tile::Animated(items, current, _)
                            if *current < (items.len() - (1 as usize)) =>
                        {
                            *current += 1
                        }
                        Tile::Animated(_, current, _) => *current = 0 as usize,

                        Tile::AnimatedOnce(items, current, _)
                            if *current < (items.len() - (1 as usize)) =>
                        {
                            *current += 1
                        }
                        Tile::AnimatedOnce(_, _, tags) => {
                            tags.retain(|tag| *tag != Tags::Barrier);
                        }
                    };
                }
            }
        }
    }

    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        for layer in 0..self.layers {
            for x in 0..self.size_x {
                for y in 0..self.size_y {
                    if let Some(texture) = self.get_tile_texture(layer, x, y) {
                        let position = Vector2 {
                            x: (x * self.tile_width) as f32 * self.scale + self.shift_x as f32,
                            y: (y * self.tile_height) as f32 * self.scale + self.shift_y as f32,
                        };
                        d.draw_texture_ex(texture, position, 0.0, self.scale, Color::WHITE);
                    }
                }
            }
        }
    }

    pub fn get_collision_tiles_with_layer(
        &self,
        layer: i32,
        other: &Rectangle,
    ) -> Option<Vec<(&Tile, Vector2)>> {
        self.map[layer as usize].get_collision_tiles(other)
    }
}

impl Collision for TiledMapLayer {
    fn collision_with_rec(&self, other: &Rectangle) -> bool {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MazeConfig {
    pub size: (i32, i32),
    pub player: (i32, i32),

    pub axes: Vec<(i32, i32)>,
    pub pickaxes: Vec<(i32, i32)>,
    pub shovels: Vec<(i32, i32)>,
    pub gears: Vec<(i32, i32)>,

    pub ground: String,
    pub objects: String,
}

impl MazeConfig {
    pub fn new(path: &str) -> Result<MazeConfig, String> {
        // Create a path to the desired file
        let path = Path::new(path);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
            Ok(_) => (),
        }

        let test: Result<MazeConfig, SpannedError> = ron::from_str(s.as_str());

        match test {
            Ok(config) => Ok(config),
            Err(why) => Err(why.to_string()),
        }
    }
}
