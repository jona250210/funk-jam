use crate::{
    item::Item,
    texture_atlas::TextureAtlas,
    tiled_map::{self, Tags, Tile, TiledMap},
    tool::Tool,
    trait_collision::Collision,
};
use raylib::prelude::*;

const SCALE: f32 = 2.0;

#[derive(Clone)]
pub struct Animation<'a> {
    frames: &'a Vec<&'a Texture2D>,
    pub current: &'a Texture2D,
    pub index: usize,
}

impl Animation<'_> {
    pub fn new<'a>(frames: &'a Vec<&Texture2D>) -> Animation<'a> {
        Animation {
            frames,
            current: &frames[0],
            index: 0,
        }
    }

    pub fn update(&mut self) {
        self.current = self.frames.get(self.index).unwrap_or(&self.frames[0]);

        self.index += 1;
        if self.index >= self.frames.len() {
            self.index = 0;
        }
    }
}

pub struct Player<'a> {
    pub pos: Vector2,
    pub dimensions: Vector2,
    pub movement: Movement,
    pub idle: Animation<'a>,
    pub run: Animation<'a>,
    orientation: Orientation,
    inventory: Inventory<'a>, // Wieso überall gleiche LIfetime, ich verstreh nichts hier ist doof und dieser Kommentar ist auch ziemlich lang irgendwie formatiert er das nicht WTH
    pub hp: i32,
}

#[derive(Clone)]
pub enum Orientation {
    Left,
    Right,
}

enum Inventory<'a> {
    Empty,
    Left(Tool<'a>),
    Right(Tool<'a>),
    Both(Tool<'a>, Tool<'a>),
}

impl<'a> Player<'a> {
    pub fn new(pos: Vector2, idle: &'a Vec<&Texture2D>, run: &'a Vec<&Texture2D>) -> Player<'a> {
        Player {
            pos,
            dimensions: Vector2::new(idle[0].width() as f32, idle[0].height() as f32),
            movement: Movement {
                direction: Vector2 { x: 0.0, y: 0.0 },
                speed: 300.0,
            },
            idle: Animation::new(idle),
            run: Animation::new(run),
            orientation: Orientation::Right,
            inventory: Inventory::Empty,
            hp: 1000,
        }
    }

    pub fn get_collision_rect(&self) -> Rectangle {
        Rectangle::new(
            self.pos.x,
            self.pos.y + 0.8 * SCALE * self.dimensions.y,
            self.dimensions.x * SCALE,
            self.dimensions.y * SCALE * 0.2,
        )
    }

    pub fn get_tool_collision_rect(&self) -> Rectangle {
        return match self.orientation {
            Orientation::Left => Rectangle::new(
                self.pos.x - 50.0,
                (self.pos.y - 6.0) + 0.5 * SCALE * self.dimensions.y,
                self.dimensions.x * SCALE * 2.0,
                self.dimensions.y * SCALE,
            ),
            Orientation::Right => Rectangle::new(
                self.pos.x + 25.0,
                (self.pos.y - 6.0) + 0.5 * SCALE * self.dimensions.y,
                self.dimensions.x * SCALE * 2.0,
                self.dimensions.y * SCALE,
            ),
        };
    }

    pub fn update(&mut self, frame_time: f32, tiled_map: &TiledMap) {
        let old_pos = self.pos.clone();
        let old_old_pos = self.pos.clone();

        self.pos.x = old_pos.x;
        self.pos.y = old_pos.y;

        self.pos += self
            .movement
            .direction
            .normalized()
            .scale_by(self.movement.speed)
            .scale_by(frame_time);

        let mut collided = false;
        let mut collision_tiles: Vec<(&Tile, Vector2)> = vec![];
        for layer in 0..tiled_map.layers {
            tiled_map
                .get_collision_tiles_with_layer(layer, &self.get_collision_rect())
                .map(|mut t| collision_tiles.append(&mut t));
        }
        for (tile, _pos) in collision_tiles {
            match tile {
                tiled_map::Tile::Static(_, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                    collided = true;
                }
                tiled_map::Tile::Animated(_, _, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                    collided = true;
                }
                tiled_map::Tile::AnimatedOnce(_, _, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                    collided = true;
                }
                _ => (),
            }
        }

        if collided {
            collided = false;
            self.pos.x = old_pos.x;
            self.pos.y = old_pos.y;
            self.pos += self
                .movement
                .direction
                .normalized()
                .scale_by(self.movement.speed)
                .scale_by(frame_time);
            self.pos.x = old_pos.x;
            let mut collision_tiles: Vec<(&Tile, Vector2)> = vec![];
            for layer in 0..tiled_map.layers {
                tiled_map
                    .get_collision_tiles_with_layer(layer, &self.get_collision_rect())
                    .map(|mut t| collision_tiles.append(&mut t));
            }
            for (tile, _pos) in collision_tiles {
                match tile {
                    tiled_map::Tile::Static(_, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.x = old_pos.x;
                        self.pos.y = old_pos.y;
                        collided = true;
                    }
                    tiled_map::Tile::Animated(_, _, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.x = old_pos.x;
                        self.pos.y = old_pos.y;
                        collided = true;
                    }
                    tiled_map::Tile::AnimatedOnce(_, _, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.x = old_pos.x;
                        self.pos.y = old_pos.y;
                        collided = true;
                    }
                    _ => (),
                }
            }
        }

        if collided {
            self.pos.x = old_pos.x;
            self.pos.y = old_pos.y;
            self.pos += self
                .movement
                .direction
                .normalized()
                .scale_by(self.movement.speed)
                .scale_by(frame_time);
            self.pos.y = old_pos.y;
            let mut collision_tiles: Vec<(&Tile, Vector2)> = vec![];
            for layer in 0..tiled_map.layers {
                tiled_map
                    .get_collision_tiles_with_layer(layer, &self.get_collision_rect())
                    .map(|mut t| collision_tiles.append(&mut t));
            }
            for (tile, _pos) in collision_tiles {
                match tile {
                    tiled_map::Tile::Static(_, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.y = old_pos.y;
                    }
                    tiled_map::Tile::Animated(_, _, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.y = old_pos.y;
                    }
                    tiled_map::Tile::AnimatedOnce(_, _, tags) if tags.contains(&Tags::Barrier) => {
                        self.pos.y = old_pos.y;
                    }
                    _ => (),
                }
            }
        }

        let mut collision_tiles: Vec<(&Tile, Vector2)> = vec![];
        for layer in 0..tiled_map.layers {
            tiled_map
                .get_collision_tiles_with_layer(layer, &self.get_collision_rect())
                .map(|mut t| collision_tiles.append(&mut t));
        }
        for (tile, _pos) in collision_tiles {
            match tile {
                tiled_map::Tile::Static(_, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                }
                tiled_map::Tile::Animated(_, _, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                }
                tiled_map::Tile::AnimatedOnce(_, _, tags) if tags.contains(&Tags::Barrier) => {
                    self.pos.x = old_pos.x;
                    self.pos.y = old_pos.y;
                }
                _ => (),
            }
        }

        self.inventory = match &self.inventory {
            Inventory::Empty => Inventory::Empty,
            Inventory::Left(l) => match l {
                Tool::Axe(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                Tool::Pickaxe(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                Tool::Shovel(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                _ => Inventory::Left(l.clone()),
            },
            Inventory::Right(r) => match r {
                Tool::Axe(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                Tool::Pickaxe(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                Tool::Shovel(orientation, animation, u, b) if *u == 0 && !b => Inventory::Empty,
                _ => Inventory::Right(r.clone()),
            },
            Inventory::Both(l, r) => match l {
                Tool::Axe(orientation, animation, u, b) if *u == 0 && !b => {
                    Inventory::Right(r.clone())
                }
                Tool::Pickaxe(orientation, animation, u, b) if *u == 0 && !b => {
                    Inventory::Right(r.clone())
                }
                Tool::Shovel(orientation, animation, u, b) if *u == 0 && !b => {
                    Inventory::Right(r.clone())
                }
                _ => match r {
                    Tool::Axe(orientation, animation, u, b) if *u == 0 && !b => {
                        Inventory::Left(l.clone())
                    }
                    Tool::Pickaxe(orientation, animation, u, b) if *u == 0 && !b => {
                        Inventory::Left(l.clone())
                    }
                    Tool::Shovel(orientation, animation, u, b) if *u == 0 && !b => {
                        Inventory::Left(l.clone())
                    }
                    _ => Inventory::Both(l.clone(), r.clone()),
                },
            },
        };
      
        if self.pos != old_old_pos {
            if self.hp.is_positive() {
                self.hp -= 1;
            } else {
                panic!("TOD");
            }
        }
    }

    pub fn use_tool(&mut self, tiled_map: &TiledMap) -> Vec<(Tile, Vector2)> {
        let mut used_tool;
        match (&self.orientation, &mut self.inventory) {
            (Orientation::Left, Inventory::Left(l)) => {
                l.use_tool();
            }
            (Orientation::Right, Inventory::Right(r)) => {
                r.use_tool();
            }
            (Orientation::Right, Inventory::Both(_, r)) => {
                r.use_tool();
            }
            (Orientation::Left, Inventory::Both(l, _)) => {
                l.use_tool();
            }
            _ => (),
        }

        let coll_rec = self.get_tool_collision_rect();
        match (&self.orientation, &mut self.inventory) {
            (Orientation::Left, Inventory::Left(l)) => used_tool = Some(l),
            (Orientation::Right, Inventory::Right(r)) => used_tool = Some(r),
            (Orientation::Right, Inventory::Both(_, r)) => used_tool = Some(r),
            (Orientation::Left, Inventory::Both(l, _)) => used_tool = Some(l),
            _ => used_tool = None,
        }

        let mut tool_collision_tiles: Vec<(&Tile, Vector2)> = vec![];
        for layer in 0..tiled_map.layers {
            tiled_map
                .get_collision_tiles_with_layer(layer, &coll_rec)
                .map(|mut tmp| tool_collision_tiles.append(&mut tmp));
        }

        let mut marked_tiles: Vec<(Tile, Vector2)> = vec![];
        for (tile, pos) in tool_collision_tiles {
            match tile {
                Tile::Static(id, tags) if tags.contains(&Tags::Destroyable) => {
                    match &mut used_tool {
                        Some(Tool::Axe(orientation, animation, u, _)) if id == &2 => {
                            marked_tiles.push((tile.clone(), pos));
                            *u = 0;
                        }
                        Some(Tool::Pickaxe(orientation, animation, u, _)) if id == &7 => {
                            marked_tiles.push((tile.clone(), pos));
                            *u = 0;
                        }
                        Some(Tool::Shovel(orientation, animation, u, _)) if id == &18 => {
                            marked_tiles.push((tile.clone(), pos));
                            *u = 0;
                        }
                        _ => (),
                    };
                }
                // We will only interact with Static Tiles?
                _ => (),
            }
        }
        return marked_tiles;
    }

    pub fn animation_update(&mut self) {
        self.idle.update();
        self.run.update();
        match &mut self.inventory {
            Inventory::Left(l) => l.update(),
            Inventory::Right(r) => r.update(),
            Inventory::Both(l, r) => {
                l.update();
                r.update();
            }
            _ => (),
        }
    }

    pub fn draw(
        &mut self,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        delta_time: f32,
        elapsed_time: f32,
    ) {
        let texture;

        if self.movement.moves() {
            texture = &self.run.current;
        } else {
            texture = &self.idle.current;
        }

        let rotation_modifier = match self.orientation {
            Orientation::Left => -1,
            Orientation::Right => 1,
        };

        d.draw_texture_pro(
            texture,
            Rectangle::new(
                0.0,
                0.0,
                (rotation_modifier * texture.width()) as f32,
                texture.height() as f32,
            ),
            Rectangle::new(
                self.pos.x,
                self.pos.y,
                (texture.width() as f32 * SCALE) as f32,
                (texture.height() as f32 * SCALE) as f32,
            ),
            Vector2::zero(),
            0 as f32,
            Color::WHITE,
        );

        let tmp = self.get_collision_rect();
        d.draw_rectangle_lines(
            tmp.x as i32,
            tmp.y as i32,
            tmp.width as i32,
            tmp.height as i32,
            Color::RED,
        );
        let tmp = self.get_tool_collision_rect();
        d.draw_rectangle_lines(
            tmp.x as i32,
            tmp.y as i32,
            tmp.width as i32,
            tmp.height as i32,
            Color::RED,
        );

        // Draw items
        let offset = 20.0;
        match &mut self.inventory {
            Inventory::Left(l) => {
                l.render(d, self.pos, elapsed_time, delta_time);
            }
            Inventory::Right(r) => {
                r.render(d, self.pos, elapsed_time, delta_time);
            }
            Inventory::Both(l, r) => {
                l.render(d, self.pos, elapsed_time, delta_time);
                r.render(d, self.pos, elapsed_time, delta_time);
            }
            _ => (),
        };
    }

    pub fn add_tool(
        &mut self,
        item: &Item<'a>,
        atlas: &TextureAtlas,
        axe_frames: &'a Vec<&Texture2D>,
        pickaxe_frames: &'a Vec<&Texture2D>,
        shovel_frames: &'a Vec<&Texture2D>,
    ) -> bool {
        let tool = match item.item_type {
            crate::item::ItemType::Axe => {
                Tool::Axe(Orientation::Left, Animation::new(&axe_frames), 1, false)
            }

            crate::item::ItemType::Pickaxe => Tool::Pickaxe(
                Orientation::Right,
                Animation::new(&pickaxe_frames),
                1,
                false,
            ),
            crate::item::ItemType::Gear => {
                self.hp = 1000;
                return true;
            }
            crate::item::ItemType::Shovel => {
                Tool::Shovel(Orientation::Right, Animation::new(&shovel_frames), 1, false)
            }
        };

        match &self.inventory {
            Inventory::Empty => {
                self.inventory = Inventory::Left(tool);
            }
            Inventory::Left(l) => {
                self.inventory = Inventory::Both(l.clone(), tool);
            }
            Inventory::Right(r) => self.inventory = Inventory::Both(tool, r.clone()),
            _ => return false,
        };

        // Guter Code :/
        match &mut self.inventory {
            Inventory::Left(l) => match l {
                Tool::Axe(or, _, _, _) => *or = Orientation::Left,
                Tool::Pickaxe(or, _, _, _) => *or = Orientation::Left,
                Tool::Shovel(or, _, _, _) => *or = Orientation::Left,
            },
            Inventory::Right(l) => match l {
                Tool::Axe(or, _, _, _) => *or = Orientation::Right,
                Tool::Pickaxe(or, _, _, _) => *or = Orientation::Right,
                Tool::Shovel(or, _, _, _) => *or = Orientation::Right,
            },
            Inventory::Both(l, r) => {
                match l {
                    Tool::Axe(or, _, _, _) => *or = Orientation::Left,
                    Tool::Pickaxe(or, _, _, _) => *or = Orientation::Left,
                    Tool::Shovel(or, _, _, _) => *or = Orientation::Left,
                }

                match r {
                    Tool::Axe(or, _, _, _) => *or = Orientation::Right,
                    Tool::Pickaxe(or, _, _, _) => *or = Orientation::Right,
                    Tool::Shovel(or, _, _, _) => *or = Orientation::Right,
                }
            }

            _ => return false,
        };

        true
    }

    pub fn up(&mut self) {
        self.movement.up();
    }

    pub fn down(&mut self) {
        self.movement.down();
    }

    pub fn left(&mut self) {
        self.movement.left();
        self.orientation = Orientation::Left;
    }

    pub fn right(&mut self) {
        self.movement.right();
        self.orientation = Orientation::Right;
    }

    pub fn switch_tools(&mut self) {
        self.inventory = match &self.inventory {
            Inventory::Empty => Inventory::Empty,
            Inventory::Left(l) => Inventory::Right(l.clone()),
            Inventory::Right(r) => Inventory::Left(r.clone()),
            Inventory::Both(l, r) => Inventory::Both(r.clone(), l.clone()),
        };

        match &mut self.inventory {
            Inventory::Left(l) => match l {
                Tool::Axe(or, _, _, _) => *or = Orientation::Left,
                Tool::Pickaxe(or, _, _, _) => *or = Orientation::Left,
                Tool::Shovel(or, _, _, _) => *or = Orientation::Left,
            },
            Inventory::Right(l) => match l {
                Tool::Axe(or, _, _, _) => *or = Orientation::Right,
                Tool::Pickaxe(or, _, _, _) => *or = Orientation::Right,
                Tool::Shovel(or, _, _, _) => *or = Orientation::Right,
            },
            Inventory::Both(l, r) => {
                match l {
                    Tool::Axe(or, _, _, _) => *or = Orientation::Left,
                    Tool::Pickaxe(or, _, _, _) => *or = Orientation::Left,
                    Tool::Shovel(or, _, _, _) => *or = Orientation::Left,
                }

                match r {
                    Tool::Axe(or, _, _, _) => *or = Orientation::Right,
                    Tool::Pickaxe(or, _, _, _) => *or = Orientation::Right,
                    Tool::Shovel(or, _, _, _) => *or = Orientation::Right,
                }
            }

            _ => (),
        };
    }
}

pub struct Movement {
    pub direction: Vector2,
    pub speed: f32,
}

impl Movement {
    pub fn reset(&mut self) {
        self.direction = Vector2::zero();
    }

    pub fn up(&mut self) {
        self.direction.y = -1 as f32;
    }

    pub fn down(&mut self) {
        self.direction.y = 1 as f32;
    }

    pub fn left(&mut self) {
        self.direction.x = -1 as f32;
    }

    pub fn right(&mut self) {
        self.direction.x = 1 as f32;
    }

    pub fn moves(&mut self) -> bool {
        self.direction != Vector2::zero()
    }
}

impl Collision for Player<'_> {
    fn collision_with_rec(&self, other: &Rectangle) -> bool {
        Rectangle::new(self.pos.x, self.pos.y, self.dimensions.x, self.dimensions.y)
            .check_collision_recs(other)
    }
}
