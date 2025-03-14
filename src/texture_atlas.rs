use raylib::prelude::*;
use std::collections::HashMap;

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

    pub fn get_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Result<&Texture2D, String> {
        if !self.textures.contains_key(path) {
            let texture: Texture2D = rl.load_texture(&thread, path).expect("Failed to load texture");
            self.textures.insert(path.to_string(), texture);
        }
        return match self.textures.get(path) {
            Some(t) => Ok(t),
            _ => Err(format!("Failed to return Texture {}", path))
        }
    }

}