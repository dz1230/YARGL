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
    html: String,
    vdom: VDom<'a>,
}

impl Window<'_> {
    fn parse_html(&mut self) {
        self.vdom = tl::parse(&self.html, tl::ParserOptions::default()).unwrap();
    }

    pub fn new<'a>(video_subsystem: &VideoSubsystem, options: &WindowCreationOptions, html_file: &str) -> Window<'a> {
        let sdl_window = video_subsystem.window(&options.title, options.width, options.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().unwrap();
        let mut w = Window {
            sdl_canvas: canvas,
            html: std::fs::read_to_string(html_file).unwrap(),
            vdom: tl::parse("", tl::ParserOptions::default()).unwrap(),
        };
        w.parse_html();
        //w.vdom = tl::parse(&w.html, tl::ParserOptions::default()).unwrap();
        w
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
