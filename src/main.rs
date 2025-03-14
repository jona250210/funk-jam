
mod player;

use nonempty::{NonEmpty, nonempty};
use player::Player;

use raylib::prelude::*;

mod audiomanager;
use audiomanager::AudioManager;

mod texture_atlas;
use texture_atlas::TextureAtlas;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut atlas = TextureAtlas::new();
    let frames = nonempty![
        rl.load_texture(&thread, "assets/run0.png").unwrap(),
        rl.load_texture(&thread, "assets/run1.png").unwrap(),
        rl.load_texture(&thread, "assets/run2.png").unwrap(),
        rl.load_texture(&thread, "assets/run3.png").unwrap(),
        rl.load_texture(&thread, "assets/run4.png").unwrap(),
        rl.load_texture(&thread, "assets/run5.png").unwrap(),
        rl.load_texture(&thread, "assets/run6.png").unwrap(),
        //atlas.get_texture(&mut rl, &thread, "assets/run0.png"),
        //atlas.get_texture(&mut rl, &thread, "assets/run1.png"),
    ];

    let mut player = Player::new(Vector2::new(200.0, 200.0), &frames);

    println!(
        "({}, {})",
        player.animation.current.width, player.animation.current.height
    );

    let mut atlas = TextureAtlas::new();
    let mut textures: Vec<&Texture2D> = Vec::new();
    let texture = atlas.get_texture(&mut rl, &thread, "Hammer.png");
    textures.push(texture);

    let l = rl.load_texture(&thread, "Hammer.png").unwrap();

    let mut audio_device = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut audio_manager = AudioManager::new(&mut audio_device);
    audio_manager.load_sound( "test", "sword_sound.wav");
    audio_manager.play_sound( "test");

    println!("({}, {})", l.width, l.height);

    while !rl.window_should_close() {
        rl.set_target_fps(120);
        
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

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture(textures[0], 100, 100, Color::WHITE);
        d.draw_texture(&l, 200, 200, Color::WHITE);


    }
}
