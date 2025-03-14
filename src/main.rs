mod player;

use nonempty::{NonEmpty, nonempty};
use player::Player;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let frames = nonempty![
        rl.load_texture(&thread, "assets/run0.png").unwrap(),
        rl.load_texture(&thread, "assets/run1.png").unwrap(),
        rl.load_texture(&thread, "assets/run2.png").unwrap(),
        rl.load_texture(&thread, "assets/run3.png").unwrap(),
        rl.load_texture(&thread, "assets/run4.png").unwrap(),
        rl.load_texture(&thread, "assets/run5.png").unwrap(),
        rl.load_texture(&thread, "assets/run6.png").unwrap(),
    ];

    rl.set_target_fps(120);
    let mut player = Player::new(Vector2::new(200.0, 200.0), &frames);

    println!(
        "({}, {})",
        player.animation.current.width, player.animation.current.height
    );

    while !rl.window_should_close() {
        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_fps(12, 12);
            d.draw_texture_ex(
                &player.animation.current,
                player.pos,
                0 as f32,
                4 as f32,
                Color::WHITE,
            );
        }

        player.movement.reset();
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.movement.up();
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.movement.down();
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.movement.right();
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.movement.left();
        }
        player.update(rl.get_frame_time());
    }
}
