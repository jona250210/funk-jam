use crate::trait_collision::Collision;
use raylib::prelude::*;

const SCALE: f32 = 2.0;

pub struct Animation<'a> {
    frames: &'a Vec<&'a Texture2D>,
    pub current: &'a Texture2D,
    index: usize,
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
}

enum Orientation {
    Left,
    Right,
}

impl Player<'_> {
    pub fn new<'a>(
        pos: Vector2,
        idle: &'a Vec<&Texture2D>,
        run: &'a Vec<&Texture2D>,
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

    pub fn update(&mut self, frame_time: f32) {
        self.pos += self
            .movement
            .direction
            .normalized()
            .scale_by(self.movement.speed)
            .scale_by(frame_time);
    }

    pub fn animation_update(&mut self) {
        self.idle.update();
        self.run.update();
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
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
