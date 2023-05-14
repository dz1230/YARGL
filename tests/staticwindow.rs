
#[test]
fn static_window() {
    use std::rc::Rc;
    use yargl::event::EventReceiver;

    let mut raw_ctx = yargl::context::init().unwrap();
    let font_data_arial = std::fs::read("res/font/arial.ttf").unwrap();
    let arial_font = yargl::font::Font::new(&font_data_arial, 0, 1);
    raw_ctx.fonts.insert("arial".to_string(), &arial_font);
    let ctx = Rc::new(raw_ctx);
    let html_filename = "res/html/demo.html";
    let html = std::fs::read_to_string(html_filename).unwrap();
    let mut all_windows = vec![];
    let mut window: yargl::window::Window = yargl::window::Window::new(ctx.clone(), &yargl::window::WindowCreationOptions { title: "Test".to_string(), width: 800, height: 600 }, html.as_str(), Some(html_filename)).unwrap();
    window.pointer_up_events.add_listener(|event, window| {
        let node = window.get_node_at(event.0.data.x, event.0.data.y);
        match node {
            Some(tl::Node::Tag(_tag)) => {
                let selector = yargl::css::Selector::complete_selector(node.unwrap());
                if let Some(id) = selector.id {
                    if id == "close_window" {
                        return  yargl::event::EventReturnCode::Quit;
                    }
                }
            },
            _ => {}
        }
        return yargl::event::EventReturnCode::Continue;
    });
    window.draw();
    all_windows.push(window);
    loop {
        match ctx.poll_events(&all_windows) {
            yargl::event::EventReturnCode::Continue => {},
            yargl::event::EventReturnCode::Cancel => {},
            yargl::event::EventReturnCode::Quit => {
                break
            }
        }
    }
}