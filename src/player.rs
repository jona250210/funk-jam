use crate::{
    tiled_map::{self, Tags, Tile, TiledMap}, tool::Tool, trait_collision::Collision
};
use raylib::prelude::*;

const SCALE: f32 = 2.0;

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
    pub tool_left: Tool<'a>,
    pub tool_right: Tool<'a>,
}

pub enum Orientation {
    Left,
    Right,
}

impl Player<'_> {
    pub fn new<'a>(
        pos: Vector2,
        idle: &'a Vec<&Texture2D>,
        run: &'a Vec<&Texture2D>,
        tool_left: Tool<'a>,
        tool_right: Tool<'a>,
    ) -> Player<'a> {
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
            tool_left: tool_left,
            tool_right: tool_right,
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
    }

    pub fn use_tool(&mut self, tiled_map: &TiledMap) -> Vec<(Tile, Vector2)> {

        self.tool_left.use_tool();
        self.tool_right.use_tool();

        let mut tool_collision_tiles: Vec<(&Tile, Vector2)> = vec![];
        for layer in 0..tiled_map.layers {
            tiled_map
                .get_collision_tiles_with_layer(layer, &self.get_tool_collision_rect())
                .map(|mut tmp| tool_collision_tiles.append(&mut tmp));
        }

        let mut marked_tiles: Vec<(Tile, Vector2)> = vec![];
        for (tile, pos) in tool_collision_tiles {
            match tile {
                Tile::Static(_, tags) if tags.contains(&Tags::Destroyable) => {
                        marked_tiles.push((tile.clone(), pos));
                    },
                // We will only interact with Static Tiles?
                _ => ()
            }
        }
        return marked_tiles;
    }

    pub fn animation_update(&mut self) {
        self.idle.update();
        self.run.update();
        self.tool_left.update();
        self.tool_right.update();
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, delta_time: f32, elapsed_time: f32) {
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

        self.tool_left.render(d, self.pos, elapsed_time, delta_time);
        self.tool_right.render(d, self.pos, elapsed_time ,delta_time);
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
