
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


    // Hier alle Texturen einfügen, die automatisch geladen werden sollen
    // Sie können dann später mit atlas.get_texture("pfad/zu/texture") abgerufen werden
    let textures = [
        "assets/run0.png",
        "assets/run1.png",
        "assets/run2.png",
        "assets/run3.png",
        "assets/run4.png",
        "assets/run5.png",
        "assets/run6.png",
        "Hammer.png"
    ];
    let mut atlas = TextureAtlas::new();
    for path in textures.iter() {
        let texture = rl.load_texture(&thread, path).unwrap();
        atlas.store_texture(path, texture);
    }

    // PLAYER
    let frames = vec![
        atlas.get_texture("assets/run0.png"),
        atlas.get_texture("assets/run1.png"),
        atlas.get_texture("assets/run2.png"),
        atlas.get_texture("assets/run3.png"),
        atlas.get_texture("assets/run4.png"),
        atlas.get_texture("assets/run5.png"),
        atlas.get_texture("assets/run6.png"),
    ];
    let mut player = Player::new(Vector2::new(200.0, 200.0), &frames);

    // DEMO
    let texture1 = atlas.get_texture("Hammer.png");
    let texture2 = atlas.get_texture("Hammer.png");
    let texture3 = atlas.get_texture("Hammer.png");

    // AUDIO MANAGER
    let mut audio_device = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut audio_manager = AudioManager::new(&mut audio_device);
    audio_manager.load_sound( "test", "sword_sound.wav");
    audio_manager.play_sound( "test");

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
        d.draw_text("Hello, world!", 42, 42, 20, Color::BLACK);
        d.draw_texture(texture1, 100, 100, Color::WHITE);
        d.draw_texture(texture2, 150, 150, Color::WHITE);
        d.draw_texture(texture3, 150, 150, Color::WHITE);
    }
}
