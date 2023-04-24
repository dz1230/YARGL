use sdl2::pixels::Color;
use std::ptr::eq;
use std::rc::Rc;

use crate::font::Font;
use crate::event::{Event, EventReceiver, EventReturnCode};

pub struct Element {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    radius: u32,
    pub enabled: bool,
    pub parent: Option<Rc<Element>>,
    children: Vec<Rc<Element>>,
}

impl Element {
    pub fn new() -> Element {
        Element {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            radius: 0,
            enabled: true,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_radius(&self) -> u32 {
        self.radius
    }
    pub fn get_inner_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x + self.radius as i32, self.y + self.radius as i32, self.width - self.radius * 2, self.height - self.radius * 2)
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &Rc<Element>> {
        self.children.iter()
    }
    pub fn get_children(&self) -> Vec<Rc<Element>> {
        self.children.clone()
    }
    pub fn set_children(&mut self, children: Vec<Rc<Element>>) {
        self.children = children;
    }
    pub fn add_child(&mut self, child: Rc<Element>) {
        self.children.push(child);
    }
    pub fn remove_child(&mut self, child: Rc<Element>) {
        self.children.retain(|c| !eq(c, &child));
    }
    pub fn add_children(&mut self, children: Vec<Rc<Element>>) {
        self.children.extend(children);
    }
}

pub struct BackgroundElement {
    pub element: Element,
    pub background_color: Color,
}

pub struct TextElement<'a> {
    pub element: Element,
    pub text: String,
    pub font: Rc<Font<'a>>,
    pub font_size: u32,
    pub font_color: Color,
}

// default implementation
impl<T> EventReceiver<T> for Element {
    fn on(&mut self, _event: &Event<T>) -> EventReturnCode {
        EventReturnCode::Continue
    }
}
