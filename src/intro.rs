use raylib::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct IntroSequence {
    pub(crate) files_content: Vec<String>,
}

impl IntroSequence {
    pub fn new(intro_dir: &str) -> Result<Self, String> {
        let mut files_content = Vec::new();
        
        match fs::read_dir(intro_dir) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(content) = Self::read_file_content(&path) {
                            files_content.push(content);
                        }
                    }
                }
                Ok(Self { files_content })
            },
            Err(e) => Err(format!("Failed to read intro directory: {}", e)),
        }
    }
    
    fn read_file_content(path: &PathBuf) -> Option<String> {
        if let Some(path_str) = path.to_str() {
            match fs::read_to_string(path_str) {
                Ok(content) => Some(content),
                Err(_) => None,
            }
        } else {
            None
        }
    }
    
    pub fn play(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> bool {
        let mut last_skip_time = 0.0;
        
        'content: for content in &self.files_content {
            let mut show_skip_message = false;
            while !rl.window_should_close() {
                let current_time = rl.get_time();
                if current_time - last_skip_time >= 3.0 {
                    show_skip_message = true;
                    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                        last_skip_time = current_time;
                        continue 'content;
                    }
                }

                let window_height = rl.get_screen_height();
                let window_width = rl.get_screen_width();

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);

                let font_size = 22;
                let screen_width = window_width;
                let line_height = font_size + 5;
                let max_width = screen_width - 80; 
                
                let words: Vec<&str> = content.split_whitespace().collect();
                let mut lines: Vec<String> = Vec::new();
                let mut current_line = String::new();
                
                for word in words {
                    let test_line = if current_line.is_empty() {
                        word.to_string()
                    } else {
                        format!("{} {}", current_line, word)
                    };
                    
                    let test_width = d.measure_text(&test_line, font_size);
                    if (test_width > max_width && !current_line.is_empty()) || word == "\\n" {
                        lines.push(current_line);
                        if word == "\\n" {
                            current_line = String::new();
                        } else {
                            current_line = word.to_string();
                        }
                    } else {
                        current_line = test_line;
                    }
                }
                
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                
                let start_y = (window_height - (lines.len() as i32 * line_height)) / 2;
                let total_text_height = lines.len() as i32 * line_height;

                for (i, line) in lines.iter().enumerate() {
                    let line_width = d.measure_text(line, font_size);
                    let x = (screen_width - line_width) / 2;
                    let y = start_y + (i as i32 * line_height);
                    d.draw_text(line, x, y, font_size, Color::WHITE);
                    if show_skip_message {
                        let skip_text = "Press SPACE to skip";
                        let skip_width = d.measure_text(skip_text, font_size);
                        let skip_x = (screen_width - skip_width) / 2;
                        let skip_y = start_y + total_text_height + 20;
                        d.draw_text(skip_text, skip_x, skip_y, font_size, Color::WHITE);
                    }
                        d.draw_text(line, x, y, font_size, Color::WHITE);
                    }
                }
            }
            
            if rl.window_should_close() {
                return false;
            }
            true
        }
    }
