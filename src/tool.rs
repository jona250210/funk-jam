use raylib::prelude::*;

use crate::player::{self, Animation, Orientation};

type UsesLeft = i32;
type AnimationRunning = bool;

#[derive(Clone)]
pub enum Tool<'a> {
    Axe(Orientation, Animation<'a>, UsesLeft, AnimationRunning),
    Pickaxe(Orientation, Animation<'a>, UsesLeft, AnimationRunning),
    Shovel(Orientation, Animation<'a>, UsesLeft, AnimationRunning),
}

const SCALE: f32 = 1.0;

impl<'a> Tool<'a> {
    pub fn use_tool(&mut self) {
        match self {
            Tool::Axe(orientation, _, _, animation_running) => {
                match orientation {
                    Orientation::Left => *animation_running = true,
                    Orientation::Right => *animation_running = true
                }
            }
            Tool::Pickaxe(orientation, _, _, animation_running) => {
                match orientation {
                    Orientation::Left => *animation_running = true,
                    Orientation::Right => *animation_running = true
                }
            }
            Tool::Shovel(orientation, _, _, animation_running) => {
                match orientation {
                    Orientation::Left => *animation_running = true,
                    Orientation::Right => *animation_running = true
                }
            }
        }
    }

    pub fn update(&mut self) {
        match self {
            Tool::Axe(_, animation, _, animation_running) => {
                if *animation_running {
                    animation.update();
                }
                if animation.index == 0 {
                    *animation_running = false;
                    
                }
            }
            Tool::Pickaxe(_, animation, _, animation_running) => {
                if *animation_running {
                    animation.update();
                }
                if animation.index == 0 {
                    *animation_running = false;
                }
            }
            Tool::Shovel(_, animation, _, animation_running) => {
                if *animation_running {
                    animation.update();
                }
                if animation.index == 0 {
                    *animation_running = false;
                }
            }
        }
    }

    pub fn render(&mut self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, player_pos: Vector2, elapsed_time: f32, delta_time: f32) {
        
        let mut tmp: (i32, &Texture2D) = match self {
            Tool::Axe(orientation, animation, _, _) => {
                match orientation {
                    Orientation::Left => (-1, animation.current),
                    Orientation::Right => (1, animation.current)
                }
            }
            Tool::Pickaxe(orientation, animation, _, _) => {
                match orientation {
                    Orientation::Left => (-1, animation.current),
                    Orientation::Right => (1, animation.current)
                }
            }
            Tool::Shovel(orientation, animation, _, _) => {
                match orientation {
                    Orientation::Left => (-1, animation.current),
                    Orientation::Right => (1, animation.current)
                }
            }
        };

        let mut x = 0.0;
        if tmp.0 == -1 {
            x = player_pos.x - 32.0;
        }
        else {
            x = player_pos.x + 64.0;
        }


        d.draw_texture_pro(
            tmp.1,
            Rectangle::new(
                0.0,
                0.0,
                (tmp.0 * tmp.1.width()) as f32,
                tmp.1.height() as f32,
            ),
            Rectangle::new(
                x,
                player_pos.y + 12.0 + ((elapsed_time*4.0).sin() * 4.0) + 7.0,
                (tmp.1.width() as f32 * SCALE) as f32,
                (tmp.1.height() as f32 * SCALE) as f32,
            ),
            Vector2::zero(),
            0 as f32,
            Color::WHITE,
        );


    }



}
