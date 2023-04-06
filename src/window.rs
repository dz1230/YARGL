use sdl2::{rect::Rect, VideoSubsystem};

use crate::element::{TextElement, Background};

pub struct WindowCreationOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window {
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(video_subsystem: &VideoSubsystem, options: &WindowCreationOptions) -> Window {
        let sdl_window = video_subsystem.window(&options.title, options.width, options.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().unwrap();
        Window {
            sdl_canvas: canvas,
        }
    }
}

trait Drawable {
    fn draw(&self, window: &mut Window);
}

impl Drawable for dyn Background {
    fn draw(&self, window: &mut Window) {
        window.sdl_canvas.set_draw_color(self.get_background_color());
        window.sdl_canvas.fill_rect(Rect::new(self.get_x(), self.get_y(), self.get_width(), self.get_height())).unwrap();
    }
}

impl Drawable for dyn TextElement {
    fn draw(&self, window: &mut Window) {
        window.sdl_canvas.set_draw_color(self.get_font_color());
        
        
        // TODO render text to texture
    }
}
