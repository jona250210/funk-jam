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

    pub fn store_texture(&mut self, path: &str, texture: Texture2D) {
        self.textures.insert(path.to_string(), texture);
    }

    pub fn get_texture(&self, path: &str) -> &Texture2D {
        if !self.textures.contains_key(path) {
            println!("Texture {} not loaded yet", path)
        }
        return self.textures.get(path).unwrap();
    }

}