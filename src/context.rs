
use std::collections::HashMap;

use crate::{event::EventReturnCode, font::Font};

pub struct Context<'a> {
    sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    fonts: HashMap<String, Font<'a>>
}

impl Context<'_> {
    pub fn new<'a>() -> Context<'a> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        Context {
            sdl,
            video_subsystem,
            fonts: HashMap::new()
        }
    }

    pub fn load_font<'b>(&'b mut self, path: &std::path::Path) {
        if !path.is_absolute() {
            match std::env::current_dir() {
                Ok(mut current_dir) => {
                    current_dir.push(path);
                    return self.load_font(current_dir.as_path());
                },
                Err(_) => {}
            }
        }
        if let Some(os_name) = path.file_stem()  {
            if let Some(name) = os_name.to_str() {
                self.fonts.insert(name.to_lowercase(), Font::new());
                let mut font = self.fonts.get_mut(name.to_lowercase().as_str()).unwrap();
                font.load_faces(path, 0, 1);
            }
        }
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name.to_lowercase().as_str())
    }

    pub fn poll_events(&self) -> EventReturnCode {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut return_code = EventReturnCode::Continue;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    return_code = EventReturnCode::Quit;
                    break
                },
                _ => {
                    // TODO handle other events
                }
            }
        }
        return_code
    }
}