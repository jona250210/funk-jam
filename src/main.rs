mod trait_collision;

mod player;
use std::vec;

use player::Player;

mod camera;
use camera::GameCamera;

use raylib::prelude::*;
mod audiomanager;
use audiomanager::AudioManager;

mod texture_atlas;
use texture_atlas::TextureAtlas;

mod tiled_map;
use tiled_map::{MazeConfig, Tags, Tile, TiledMap, SCALE, TILE_HEIGHT, TILE_WIDTH};

mod item;
use item::Item;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let test: MazeConfig = match MazeConfig::new("assets/maze0.KB") {
        Ok(config) => config,
        Err(why) => panic!("{}", why),
    };
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
        "assets/idle0.png",
        "assets/idle1.png",
        "assets/idle2.png",
        "assets/idle3.png",
        "assets/idle4.png",
        "assets/idle5.png",
        "assets/idle6.png",
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
        "assets/potion.png",
        "assets/water0.png",
        "assets/water1.png",
        "assets/water2.png",
        "assets/water3.png",
        "assets/Sandmauer.png",
    ];
    let mut atlas = TextureAtlas::new();
    for path in textures.iter() {
        let texture = rl.load_texture(&thread, path).unwrap();
        atlas.store_texture(path, texture);
    }

    // PLAYER
    let run_frames = vec![
        atlas.get_texture("assets/run0.png"),
        atlas.get_texture("assets/run1.png"),
        atlas.get_texture("assets/run2.png"),
        atlas.get_texture("assets/run3.png"),
        atlas.get_texture("assets/run4.png"),
        atlas.get_texture("assets/run5.png"),
        atlas.get_texture("assets/run6.png"),
    ];

    let idle_frames = vec![
        atlas.get_texture("assets/idle0.png"),
        atlas.get_texture("assets/idle1.png"),
        atlas.get_texture("assets/idle2.png"),
        atlas.get_texture("assets/idle3.png"),
        atlas.get_texture("assets/idle4.png"),
        atlas.get_texture("assets/idle5.png"),
        atlas.get_texture("assets/idle6.png"),
    ];

    let mut player = Player::new(
        Vector2::new(
            (test.player.0 * TILE_WIDTH) as f32 + TILE_WIDTH as f32 / 2.0,
            (test.player.1 * TILE_HEIGHT) as f32 + TILE_HEIGHT as f32 / 2.0,
        ),
        &idle_frames,
        &run_frames,
    );

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
    audio_manager.load_sound("test2", "assets/sounds/stone.ogg");
    audio_manager.play_sound("test2");

    let mut frame_times = 0 as f32;

    // TILED MAP
    // let mut tiled_map: TiledMap<'_> = TiledMap::new(5, 20, 20, &atlas);
    let mut tiled_map = match TiledMap::from(test, &atlas) {
        Ok(map) => map,
        Err(why) => panic!("Error: {}", why),
    };

    // ITEMS
    let mut items = vec![
        Item::new(
            Vector2::new(100.0, 100.0),
            atlas.get_texture("assets/potion.png"),
            1.25,
        ),
        Item::new(
            Vector2::new(200.0, 200.0),
            atlas.get_texture("assets/potion.png"),
            1.25,
        ),
    ];

    rl.set_target_fps(120);
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        player.movement.reset();
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.up();
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.down();
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.right();
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.left();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let marked_tiles:Vec<(Tile, Vector2)> = player.use_tool(&tiled_map);
            tiled_map.handle_hit_tiles(marked_tiles);
        }

        player.update(delta_time, &tiled_map);

        // Update camera target to follow player
        game_camera.update_target(player.pos, 20.0, 20.0);

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            let mut d = d.begin_mode2D(game_camera.camera);
            tiled_map.update_animated_tiles(delta_time);
            tiled_map.render(&mut d);

            for item in items.iter() {
                item.render(&mut d);
            }

            d.draw_fps(12, 12);
            d.clear_background(Color::WHITE);
            d.draw_fps(12, 12);

            player.draw(&mut d);
        }

        if frame_times > 0.08 {
            player.animation_update();
            frame_times = 0 as f32;
        } else {
            frame_times += rl.get_frame_time()
        }
    }
}
