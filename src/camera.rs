use raylib::prelude::*;

pub struct GameCamera {
    pub camera: Camera2D,
}

impl GameCamera {
    pub fn new(screen_width: i32, screen_height: i32, initial_target: Vector2) -> Self {
        let camera = Camera2D {
            target: initial_target,
            offset: Vector2 {
                x: screen_width as f32 / 2.0,
                y: screen_height as f32 / 2.0,
            },
            rotation: 0.0,
            zoom: 1.2,
        };

        GameCamera { camera }
    }

    pub fn update_target(&mut self, target_position: Vector2, offset_x: f32, offset_y: f32) {
        self.camera.target = Vector2 {
            x: target_position.x + offset_x,
            y: target_position.y + offset_y,
        };
    }
}