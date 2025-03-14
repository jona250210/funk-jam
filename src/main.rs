mod audiomanager;

use raylib::prelude::*;

use audiomanager::AudioManager;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let l = rl.load_texture(&thread, "Hammer.png").unwrap();

    let mut audio_device = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut audio_manager = AudioManager::new(&mut audio_device);
    audio_manager.load_sound( "test", "sword_sound.wav");
    audio_manager.play_sound( "test");

    println!("({}, {})", l.width, l.height);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture(&l, 200, 200, Color::WHITE);
    }
}
