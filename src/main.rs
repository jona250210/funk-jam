mod player;
use player::Player;

mod camera;
use camera::GameCamera;

use raylib::prelude::*;

mod audiomanager;
use audiomanager::AudioManager;

mod texture_atlas;
use texture_atlas::TextureAtlas;

mod tiled_map;
use tiled_map::TiledMap;

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
        "assets/palme0.png",
        "assets/palme1.png",
        "assets/palme2.png",
        "assets/palme3.png",
        "assets/stein0.png",
        "assets/stein1.png",
        "assets/stein2.png",
        "assets/stein3.png",
        "assets/stein4.png",
        "assets/empty_tile.png",
        "assets/sand_tile.png",
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

    // CAMERA
    let mut game_camera = GameCamera::new(
        rl.get_screen_width(),
        rl.get_screen_height(),
        Vector2 {
            x: player.pos.x + 20.0,
            y: player.pos.y + 20.0,
        },
    );

    // AUDIO MANAGER
    let mut audio_device =
        RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut audio_manager = AudioManager::new(&mut audio_device);
    audio_manager.load_sound("test", "sword_sound.wav");
    audio_manager.play_sound("test");

    let mut frame_times = 0 as f32;

    // TILED MAP
    let mut tiled_map = TiledMap::new(5, 10, 10);
    let tiles = vec![
        atlas.get_texture("assets/empty_tile.png"),
        atlas.get_texture("assets/sand_tile.png"),
        atlas.get_texture("assets/palme0.png"),
        atlas.get_texture("assets/palme1.png"),
        atlas.get_texture("assets/palme2.png"),
        atlas.get_texture("assets/palme3.png"),
        atlas.get_texture("assets/stein0.png"),
        atlas.get_texture("assets/stein1.png"),
        atlas.get_texture("assets/stein2.png"),
        atlas.get_texture("assets/stein3.png"),
        atlas.get_texture("assets/stein4.png"),
    ];
    for (id, tile) in tiles.iter().enumerate() {
        tiled_map.add_tile_texture(id as i32, tile);
    }

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

        // Update camera target to follow player
        game_camera.update_target(player.pos, 20.0, 20.0);

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            let mut d = d.begin_mode2D(game_camera.camera);
            tiled_map.render(&mut d);

            d.draw_fps(12, 12);
            d.draw_texture_ex(
                &player.animation.current,
                player.pos,
                0 as f32,
                4 as f32,
                Color::WHITE,
            );
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

        if frame_times > 0.08 {
            player.animation_update();
            frame_times = 0 as f32;
        } else {
            frame_times += rl.get_frame_time()
        }
    }
}
