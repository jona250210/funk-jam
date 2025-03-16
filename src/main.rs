mod trait_collision;

mod player;
use std::{ops::Deref, vec};

use player::{Animation, Player};

mod camera;
use camera::GameCamera;

use raylib::prelude::*;
mod audiomanager;
use audiomanager::AudioManager;

mod texture_atlas;
use texture_atlas::TextureAtlas;

mod tiled_map;
use tiled_map::{MazeConfig, SCALE, TILE_HEIGHT, TILE_WIDTH, Tags, Tile, TiledMap};

mod item;
use item::Item;
use trait_collision::Collision;

mod tool;
use tool::Tool;

mod intro;
use intro::IntroSequence;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    // AUDIO MANAGER
    let mut audio_device =
        RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    unsafe {
        ffi::SetAudioStreamBufferSizeDefault(4096);
    }
    let mut audio_manager: AudioManager = AudioManager::new(&mut audio_device);

    audio_manager.load_sound("hit_stone", "assets/sounds/stone.ogg");
    audio_manager.load_sound("hit_wood", "assets/sounds/wood.wav");
    audio_manager.load_sound("hit_sand", "assets/sounds/sand.ogg");
    audio_manager.load_sound("step_sand_1", "assets/sounds/sand_step_1.wav");
    audio_manager.load_sound("step_sand_2", "assets/sounds/sand_step_2.wav");
    audio_manager.load_sound("ui", "assets/sounds/menu.wav");


    let test: MazeConfig = match MazeConfig::new("assets/maze2.KB") {
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
        "assets/idle0_80.png",
        "assets/idle1_80.png",
        "assets/idle2_80.png",
        "assets/idle3_80.png",
        "assets/idle0_60.png",
        "assets/idle1_60.png",
        "assets/idle2_60.png",
        "assets/idle3_60.png",
        "assets/idle0_40.png",
        "assets/idle1_40.png",
        "assets/idle2_40.png",
        "assets/idle3_40.png",
        "assets/idle0_20.png",
        "assets/idle1_20.png",
        "assets/idle2_20.png",
        "assets/idle3_20.png",

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
        "assets/axe0.png",
        "assets/axe1.png",
        "assets/axe2.png",
        "assets/hammer0.png",
        "assets/hammer1.png",
        "assets/hammer2.png",
        "assets/pickaxe0.png",
        "assets/pickaxe1.png",
        "assets/pickaxe2.png",
        "assets/shovel0.png",
        "assets/shovel1.png",
        "assets/shovel2.png",
        "assets/shovel3.png",
        "assets/gear.png",
        "assets/pile0.png",
        "assets/pile1.png",
        "assets/pile2.png",
        "assets/pile3.png",
        "assets/pile4.png",
        "assets/pile5.png",
        "assets/pfutze.png",
    ];

    let mut atlas = TextureAtlas::new();
    for path in textures.iter() {
        let texture = rl.load_texture(&thread, path).unwrap();
        atlas.store_texture(path, texture);
    }

    // PLAYER
    let run_frames0 = vec![
        atlas.get_texture("assets/run0.png"),
        atlas.get_texture("assets/run1.png"),
        atlas.get_texture("assets/run2.png"),
        atlas.get_texture("assets/run3.png"),
        atlas.get_texture("assets/run4.png"),
        atlas.get_texture("assets/run5.png"),
        atlas.get_texture("assets/run6.png"),
    ];

    let idle_frames0 = vec![
        atlas.get_texture("assets/idle0.png"),
        atlas.get_texture("assets/idle1.png"),
        atlas.get_texture("assets/idle2.png"),
        atlas.get_texture("assets/idle3.png"),
        atlas.get_texture("assets/idle4.png"),
        atlas.get_texture("assets/idle5.png"),
        atlas.get_texture("assets/idle6.png"),
    ];

    let idle_frames1 = vec![
        atlas.get_texture("assets/idle0_80.png"),
        atlas.get_texture("assets/idle1_80.png"),
        atlas.get_texture("assets/idle2_80.png"),
        atlas.get_texture("assets/idle3_80.png"),
    ];

    let idle_frames2 = vec![
        atlas.get_texture("assets/idle0_60.png"),
        atlas.get_texture("assets/idle1_60.png"),
        atlas.get_texture("assets/idle2_60.png"),
        atlas.get_texture("assets/idle3_60.png"),
    ];

    let idle_frames3 = vec![
        atlas.get_texture("assets/idle0_40.png"),
        atlas.get_texture("assets/idle1_40.png"),
        atlas.get_texture("assets/idle2_40.png"),
        atlas.get_texture("assets/idle3_40.png"),
    ];

    let idle_frames4 = vec![
        atlas.get_texture("assets/idle0_20.png"),
        atlas.get_texture("assets/idle1_20.png"),
        atlas.get_texture("assets/idle2_20.png"),
        atlas.get_texture("assets/idle3_20.png"),
    ];

    let axe_frames = vec![
        atlas.get_texture("assets/axe0.png"),
        atlas.get_texture("assets/axe1.png"),
        atlas.get_texture("assets/axe2.png"),
    ];
    let pickaxe_frames = vec![
        atlas.get_texture("assets/pickaxe0.png"),
        atlas.get_texture("assets/pickaxe1.png"),
        atlas.get_texture("assets/pickaxe2.png"),
    ];
    let shovel_frames = vec![
        atlas.get_texture("assets/shovel0.png"),
        atlas.get_texture("assets/shovel1.png"),
        atlas.get_texture("assets/shovel2.png"),
        atlas.get_texture("assets/shovel3.png"),
    ];

    let mut player = Player::new(
        Vector2::new(
            ((test.player.0 * TILE_WIDTH) as f32 + TILE_WIDTH as f32 / 2.0) * SCALE,
            ((test.player.1 * TILE_HEIGHT) as f32 + TILE_HEIGHT as f32 / 2.0) * SCALE,
        ),
        (
            &idle_frames0,
            &idle_frames1,
            &idle_frames2,
            &idle_frames3,
            &idle_frames4,
        ),
        (
            &run_frames0,
            &run_frames0,
            &run_frames0,
            &run_frames0,
            &run_frames0,
        ),
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

    
    let mut frame_times = 0 as f32;

    // TILED MAP
    // let mut tiled_map: TiledMap<'_> = TiledMap::new(5, 20, 20, &atlas);
    let mut tiled_map = match TiledMap::from(&test, &atlas) {
        Ok(map) => map,
        Err(why) => panic!("Error: {}", why),
    };
    let mut background_tiled_map = TiledMap::water(1, 50, 50, &atlas);

    // ITEMS
    let mut items: Vec<Item> = Vec::new();

    for pickaxe in test.pickaxes {
        items.push(Item::new(
            Vector2::new(
                (pickaxe.0 * TILE_WIDTH) as f32 * SCALE,
                (pickaxe.1 * TILE_HEIGHT) as f32 * SCALE,
            ),
            atlas.get_texture("assets/pickaxe0.png"),
            1.0,
            item::ItemType::Pickaxe,
        ));
    }

    for pickaxe in test.axes {
        items.push(Item::new(
            Vector2::new(
                (pickaxe.0 * TILE_WIDTH) as f32 * SCALE,
                (pickaxe.1 * TILE_HEIGHT) as f32 * SCALE,
            ),
            atlas.get_texture("assets/axe0.png"),
            1.0,
            item::ItemType::Axe,
        ));
    }

    for pickaxe in test.shovels {
        items.push(Item::new(
            Vector2::new(
                (pickaxe.0 * TILE_WIDTH) as f32 * SCALE,
                (pickaxe.1 * TILE_HEIGHT) as f32 * SCALE,
            ),
            atlas.get_texture("assets/shovel0.png"),
            1.0,
            item::ItemType::Shovel,
        ));
    }

    for pickaxe in test.gears {
        items.push(Item::new(
            Vector2::new(
                (pickaxe.0 * TILE_WIDTH) as f32 * SCALE,
                (pickaxe.1 * TILE_HEIGHT) as f32 * SCALE,
            ),
            atlas.get_texture("assets/gear.png"),
            1.0,
            item::ItemType::Gear,
        ));
    }

    let intro = match IntroSequence::new("assets/intro") {
        Ok(intro) => intro,
        Err(err) => {
            println!("Failed to load intro sequence: {}", err);
            IntroSequence {
                files_content: Vec::new(),
            }
        }
    };

    // INTRO, WIEDER EINKOMMENTIEREN!
    if !intro.play(&mut rl, &thread, &mut audio_manager) {
       return;  // Exit if window was closed during intro
    }

    let mut elapsed_time = 0.0;

    rl.set_target_fps(120);
    let mut walk_sound_counter = 0.0;
    let mut walk_sound_switch = false;
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        elapsed_time += delta_time;
        walk_sound_counter += delta_time;

        player.movement.reset();
        let mut walking = false;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.up();
            walking = true;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.down();
            walking = true;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.right();
            walking = true;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.left();
            walking = true;
        }
        if walking && walk_sound_counter > 0.25{
            walk_sound_counter = 0.0;
            walk_sound_switch = !walk_sound_switch;
            match walk_sound_switch {
                true => audio_manager.play_sound("step_sand_1"),
                false => audio_manager.play_sound("step_sand_2"),
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let marked_tiles: Vec<(Tile, Vector2)> = player.use_tool(&tiled_map, &mut audio_manager);
            tiled_map.handle_hit_tiles(marked_tiles);
        } 
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            player.switch_tools();
        }

        let finish = player.update(delta_time, &tiled_map);

        if finish {
            break;
        }

        // Item collisions
        let player_dings = player.get_collision_rect();

        let collided_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.collision_with_rec(&player_dings))
            .map(|(index, _)| index)
            .collect();

        for &index in collided_indices.iter() {
            if player.add_tool(
                &items[index],
                &atlas,
                &axe_frames,
                &pickaxe_frames,
                &shovel_frames,
            ) {
                items.remove(index);
            }
        }

        // Update camera target to follow player
        game_camera.update_target(player.pos, 20.0, 20.0);

        {
            let mut dh = rl.begin_drawing(&thread);

            dh.clear_background(Color::WHITE);

            let mut d = dh.begin_mode2D(game_camera.camera);
            background_tiled_map.update_animated_tiles(delta_time);
            background_tiled_map.render(&mut d);
            tiled_map.update_animated_tiles(delta_time);
            tiled_map.render(&mut d);

            for item in items.iter() {
                item.render(&mut d);
            }

            d.draw_fps(12, 12);
            d.draw_text(format!("HP: {}", player.hp).as_str(), (player.pos.x - 100.0) as i32, (player.pos.y + 50.0) as i32, 30, Color::RED);
            d.clear_background(Color::WHITE);
            d.draw_fps(12, 12);

            player.draw(&mut d, delta_time, elapsed_time);
            
        }

        if frame_times > 0.12 {
            player.animation_update();
            frame_times = 0 as f32;
        } else {
            frame_times += rl.get_frame_time()
        }
    }

    // Endscreen
    let outro = match IntroSequence::new("assets/outro") {
        Ok(outro) => outro,
        Err(err) => {
            println!("Failed to load outro sequence: {}", err);
            IntroSequence { files_content: Vec::new() }
        }
    };
    if !outro.play(&mut rl, &thread) {
       return;  // Exit if window was closed during outro
    }
}
