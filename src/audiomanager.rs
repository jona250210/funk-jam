use raylib::prelude::*;
use raylib::core::audio::{Sound, RaylibAudio};
use std::collections::HashMap;

pub struct AudioManager<'a> {
    audio_device: &'a RaylibAudio,
    sounds: HashMap<String, Sound<'a>>,
}

impl<'a> AudioManager<'a> {
    pub fn new(audio_device: &'a RaylibAudio) -> Self {
        AudioManager {
            audio_device,
            sounds: HashMap::new(),
        }
    }

    pub fn load_sound(&mut self, name: &str, path: &str) {
        match self.audio_device.new_sound(path) {
            Ok(sound) => {
                self.sounds.insert(name.to_string(), sound);
            }
            Err(e) => {
                eprintln!("Failed to load sound {}: {}", path, e);
            }
        }
    }

    pub fn play_sound(&mut self, name: &str) {
        if let Some(sound) = self.sounds.get_mut(name) {
            self.sounds.get(name).unwrap().play();
        } else {
            eprintln!("Sound {} not found", name);
        }
    }
}