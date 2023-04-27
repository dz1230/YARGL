use sdl2::VideoSubsystem;
use tl::VDom;

use crate::element::{TextElement, BackgroundElement};

pub struct WindowCreationOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window<'a> {
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    vdom: VDom<'a>,
}

impl Window<'_> {
    pub fn new<'a>(video_subsystem: &VideoSubsystem, options: &WindowCreationOptions, html: &'a str) -> Window<'a> {
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
            vdom: tl::parse(html, tl::ParserOptions::default()).unwrap(),
        }
    }

    pub fn draw(&self) {
        let children = self.vdom.children();
        if children.len() != 1 {
            panic!("Window must have exactly one child");
        }
        self.draw_element(children.last().unwrap());
    }

    fn draw_element(&self, node_handle: &tl::NodeHandle) {
        let node = node_handle.get(self.vdom.parser()).unwrap();
        // Compute style
        
        // Draw background

        // Draw text
        let _text = node.inner_text(self.vdom.parser()).to_string();
        
        // Draw children
        match node.children() {
            Some(children) => {
                for child in children.top().iter() {
                    self.draw_element(child);
                }
            },
            None => {}
        }
    }
}

trait Drawable {
    fn draw(&self, window: &mut Window);
}

impl Drawable for BackgroundElement {
    fn draw(&self, window: &mut Window) {
        window.sdl_canvas.set_draw_color(self.background_color);
        window.sdl_canvas.fill_rect(self.element.get_inner_rect()).unwrap();
    }
}

impl Drawable for TextElement<'_> {
    fn draw(&self, window: &mut Window) {
        window.sdl_canvas.set_draw_color(self.font_color);
        self.font.render(&mut window.sdl_canvas, &self.text);
    }
}
