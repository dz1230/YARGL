use std::rc::Rc;

use crate::element::{Background, Element, Node, TextElement};

pub struct HTMLElement {
    parent: Option<Rc<dyn Node>>,
    children: Vec<Rc<dyn Node>>,

}

impl Node for HTMLElement {
    fn get_parent(&self) -> Option<Rc<dyn Node>> {
        self.parent
    }

    fn get_children(&self) -> Vec<Rc<dyn Node>> {
        self.children
    }

    fn set_parent(&mut self, parent: Option<Rc<dyn Node>>) {
        self.parent = parent;
    }

    fn set_children(&mut self, children: Vec<Rc<dyn Node>>) {
        self.children = children;
    }

    fn add_child(&mut self, child: Rc<dyn Node>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, child: Rc<dyn Node>) {
        self.children.retain(|c| c != &child);
    }
}
