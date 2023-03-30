use sdl2::{render::{Texture, TextureCreator, Canvas}, surface::Surface};


pub struct Font {
    
}

impl Font {
    pub fn new() -> Font {
        Font {}
    }

    pub fn load(&mut self, _path: &str) {
        // TODO
    }

    pub fn render(&self, canvas: &Canvas<sdl2::video::Window>, _text: &str) -> Surface {
        // TODO
        
    }
}