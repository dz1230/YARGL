pub mod util;
pub mod event;
pub mod font;
pub mod element;
pub mod window;
pub mod context;
pub mod css;
pub mod input;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{context::Context, event::EventReturnCode, window::{WindowCreationOptions, Window}};

    //use super::*;

    #[test]
    fn it_works() {
        let mut raw_ctx = Context::new();
        raw_ctx.load_font(std::path::Path::new("C:\\Windows\\Fonts\\arial.ttf"));
        let ctx = Rc::new(raw_ctx);
        let html_filename = "res/html/test_1.html";
        let html = std::fs::read_to_string(html_filename).unwrap();
        let mut _window = Window::new(ctx.clone(), &WindowCreationOptions { title: "Test".to_string(), width: 800, height: 600 }, html.as_str(), Some(html_filename));
        _window.draw();
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
