
pub mod util;
pub mod event;
pub mod font;
pub mod window;
pub mod context;
pub mod css;
pub mod input;
pub mod layout;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{context::Context, event::{EventReturnCode, EventReceiver}, window::{WindowCreationOptions, Window}, font::Font};

    //use super::*;

    #[test]
    fn it_works() {
        let mut raw_ctx = Context::new();
        let font_data_arial = std::fs::read("C:\\Windows\\Fonts\\arial.ttf").unwrap();
        let arial_font = Font::new(&font_data_arial, 0, 1);
        raw_ctx.fonts.insert("arial".to_string(), &arial_font);
        let ctx = Rc::new(raw_ctx);
        let html_filename = "res/html/demo.html";
        let html = std::fs::read_to_string(html_filename).unwrap();
        let mut all_windows = vec![];
        let mut window: Window = Window::new(ctx.clone(), &WindowCreationOptions { title: "Test".to_string(), width: 800, height: 600 }, html.as_str(), Some(html_filename));
        window.pointer_up_events.add_listener(|event, window| {
            let node = window.get_node_at(event.0.data.x, event.0.data.y);
            match node {
                None => {
                    println!("Clicked: None");
                },
                Some(tl::Node::Tag(node)) => {
                    println!("Clicked: {:?}", node.name());
                },
                Some(tl::Node::Raw(node)) => {
                    println!("Clicked: {:?}", node);
                },
                _ => {}
            }
            return EventReturnCode::Continue;
        });
        window.draw();
        all_windows.push(window);
        loop {
            match ctx.poll_events(&all_windows) {
                EventReturnCode::Continue => {},
                EventReturnCode::Cancel => {},
                EventReturnCode::Quit => {
                    break
                }
            }
        }
    }
}
