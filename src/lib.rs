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
    use crate::{context::Context, event::EventReturnCode, window::WindowCreationOptions, font::Font};

    //use super::*;

    #[test]
    fn it_works() {
        let mut ctx = Context::new();
        let font_filename = "C:\\Windows\\Fonts\\arial.ttf";
        let _font = Font::new(font_filename, None, None);
        let _window = ctx.create_window(&WindowCreationOptions {
            title: "Test".to_string(),
            width: 800,
            height: 600,
        }, "res/html/test_1.html");
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
