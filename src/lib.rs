pub mod util;
pub mod event;
pub mod font;
pub mod element;
pub mod window;
pub mod context;
pub mod html;
pub mod css;
pub mod input;

#[cfg(test)]
mod tests {
    use crate::{context::Context, event::EventReturnCode, window::{WindowCreationOptions, Window}, font::Font};

    //use super::*;

    #[test]
    fn it_works() {
        let ctx = Context::new();
        let font_filename = "C:\\Windows\\Fonts\\arial.ttf";
        let _font = Font::new(font_filename, None, None);
        let html = std::fs::read_to_string("res/html/test_1.html").unwrap();
        let _window = Window::new(&ctx.video_subsystem, &WindowCreationOptions { title: "Test".to_string(), width: 800, height: 600 }, html.as_str());
        loop {
            match ctx.poll_events() {
                EventReturnCode::Continue => {},
                EventReturnCode::Cancel => {},
                EventReturnCode::Quit => {
                    break
                }
            }
        }
    }
}
