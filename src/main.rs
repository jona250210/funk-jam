use std::ffi::CString;

use raylib::{
    ffi::{LoadImage, LoadTexture},
    prelude::*,
};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let l = rl.load_texture(&thread, "Hammer.png").unwrap();

    println!("({}, {})", l.width, l.height);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture(&l, 200, 200, Color::WHITE);
    }
}

fn main_alt() {
    let l = vec![
        "Hallo".to_string(),
        "Welt".to_string(),
        "Rust".to_string(),
        "ja".to_string(),
    ];

    // let _ = match get_longest(&l) {
    //     Some(a) => a,
    //     None => String::from(""),
    // };

    println!("{}", get_longest(&l).unwrap_or("!!Empty!!".to_string()));
    println!("{}", get_longest(&l).unwrap_or("!!Empty!!".to_string()));
    println!("{}", get_longest(&l).unwrap_or("!!Empty!!".to_string()));

    let hmm = Test::Name(String::from("Welt"));

    println!(
        "{}",
        match hmm {
            Test::Empty => String::from(""),
            Test::Coord { x, y } => format!("({}, {})", x, y),
            Test::Name(s) => s,
        }
    );
}

fn get_longest(l: &Vec<String>) -> Option<String> {
    let mut current: Option<&String> = None;

    for s in l {
        if s.len() > current.map_or(0, |s| s.len()) {
            current = Some(s);
        }
    }

    return current.cloned();
}

enum Test {
    Empty,
    Coord { x: i32, y: i32 },
    Name(String),
}
