use raylib::prelude::*;
use std::{collections::HashMap, error};

pub struct TextureAtlas {
    textures: HashMap<String, Texture2D>,
}

impl TextureAtlas {
    pub fn new() -> Self {
        println!("TEXTURE ATLAS CREATED");
        TextureAtlas {
            textures: HashMap::new(),
        }        
    }

    pub fn get_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> &Texture2D {
        if !self.textures.contains_key(path) {
            let texture: Texture2D = rl.load_texture(&thread, path).expect("Failed to load texture");
            self.textures.insert(path.to_string(), texture);
        }
        return self.textures.get(path).unwrap();
    }

}