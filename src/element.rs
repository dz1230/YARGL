use sdl2::{pixels::Color, render::Texture};
use std::rc::Rc;

use crate::font::Font;
use crate::event::{Event, EventReceiver, EventReturnCode};

pub trait Node {
    fn get_parent(&self) -> Option<Rc<dyn Node>>;
    fn get_children(&self) -> Vec<Rc<dyn Node>>;

    fn set_parent(&mut self, parent: Option<Rc<dyn Node>>);
    fn set_children(&mut self, children: Vec<Rc<dyn Node>>);
    fn add_child(&mut self, child: Rc<dyn Node>);
    fn remove_child(&mut self, child: Rc<dyn Node>);
}

pub trait Element: Node {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_radius(&self) -> u32;
    fn is_enabled(&self) -> bool;

    fn set_x(&mut self, x: i32);
    fn set_y(&mut self, y: i32);
    fn set_width(&mut self, width: u32);
    fn set_height(&mut self, height: u32);
    fn set_radius(&mut self, radius: u32);
    fn set_enabled(&mut self, enabled: bool);
}

// default implementation
impl<T> EventReceiver<T> for dyn Element {
    fn on(&mut self, _event: &Event<T>) -> EventReturnCode {
        EventReturnCode::Continue
    }
}

pub trait Background: Element {
    fn get_background_color(&self) -> Color;
    fn get_image(&self) -> Rc<Texture>;

    fn set_background_color(&mut self, background_color: Color);
    fn set_image(&mut self, image: Rc<Texture>);
}

pub trait TextElement: Element {
    fn get_text(&self) -> String;
    fn get_font(&self) -> Rc<Font>;
    fn get_font_size(&self) -> u32;
    fn get_font_color(&self) -> Color;

    fn set_text(&mut self, text: String);
    fn set_font(&mut self, font: Rc<Font>);
    fn set_font_size(&mut self, font_size: u32);
    fn set_font_color(&mut self, font_color: Color);
}
