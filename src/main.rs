mod audiomanager;

use raylib::prelude::*;

use audiomanager::AudioManager;

mod texture_atlas;
use texture_atlas::TextureAtlas;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let l = rl.load_texture(&thread, "Hammer.png").unwrap();

    let mut audio_device = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut audio_manager = AudioManager::new(&mut audio_device);
    audio_manager.load_sound( "test", "sword_sound.wav");
    audio_manager.play_sound( "test");

    println!("({}, {})", l.width, l.height);

    let mut atlas = TextureAtlas::new();
    let mut textures: Vec<&Texture2D> = Vec::new();
    let texture = atlas.get_texture(&mut rl, &thread, "Hammer.png").unwrap();
    textures.push(texture);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture(textures[0], 100, 100, Color::WHITE);
        d.draw_texture(&l, 200, 200, Color::WHITE);
    }
}
