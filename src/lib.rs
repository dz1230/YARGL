pub mod util;
pub mod event;
pub mod font;
pub mod element;
pub mod window;
pub mod context;

#[cfg(test)]
mod tests {
    use crate::{context::Context, event::EventReturnCode, window::WindowCreationOptions};

    //use super::*;

    #[test]
    fn it_works() {
        let mut ctx = Context::new();
        let _window = ctx.create_window(&WindowCreationOptions {
            title: "Test".to_string(),
            width: 800,
            height: 600,
        });
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
