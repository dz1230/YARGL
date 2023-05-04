use std::{collections::HashMap, rc::Rc};

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
    vdom: VDom<'a>,
    all_styles: Vec<Rc<crate::css::Style>>,
    computed_styles: HashMap<tl::NodeHandle, crate::css::ComputedStyle>,
}

impl Window<'_> {
    /// Parses all styles from the document and stores them in all_styles. all_styles is cleared before parsing.
    /// - html_filename: The path to the html file. Used to resolve relative paths in link tags.
    fn read_styles(&mut self, html_filename: Option<&str>) {
        self.all_styles.clear();
        match self.vdom.query_selector("style") {
            Some(style_nodes) => {
                for node_handle in style_nodes {
                    let style_node = node_handle.get(self.vdom.parser()).unwrap();
                    let style_text = style_node.inner_text(self.vdom.parser()).to_string();
                    self.all_styles.append(&mut crate::css::parse_css(style_text.as_str()));
                }
            },
            None => {}
        }

        match self.vdom.query_selector("link[rel=\"stylesheet\"]") {
            Some(link_nodes) => {
                for node_handle in link_nodes {
                    let link_node = node_handle.get(self.vdom.parser()).unwrap();
                    let href_opt = match link_node.as_tag() {
                        Some(link_tag) => link_tag.attributes().get("href"),
                        None => continue
                    };
                    let path_opt = match href_opt {
                        Some(href_value_opt) => match href_value_opt {
                            Some(href) => href.try_as_utf8_str(),
                            None => continue
                        },
                        None => continue
                    };
                    if path_opt.is_none() {
                        continue;
                    }
                    let mut path: std::path::PathBuf;
                    let mut path_str = path_opt.unwrap();
                    if path_str.starts_with(".") && html_filename.is_some() {
                        path = std::path::PathBuf::from(html_filename.unwrap());
                        path.pop();
                        path.push(path_str);
                    } else {
                        path = std::path::PathBuf::from(path_str);
                    }
                    path_str = path.to_str().unwrap();
                    match std::fs::read_to_string(path_str) {
                        Ok(css_text) => {
                            self.all_styles.append(&mut crate::css::parse_css(css_text.as_str()));
                        },
                        Err(_) => continue
                    }
                }
            },
            None => {}
        }
    }
    
    /// Returns a vector of all node handles in the document. Sorted in tree order (root, child 1 of root, child 1 of child 1 of root, child 2 of child 1 of root, child 2 of root...).
    /// Call with self.vdom.children() to get all node handles in the document.
    fn get_all_handles<'a, I>(&self, nodes: I) -> Vec<tl::NodeHandle>
    where I: IntoIterator<Item = &'a tl::NodeHandle> {
        let mut result: Vec<tl::NodeHandle> = Vec::new();
        for node_handle in nodes {
            result.push(*node_handle);
            match node_handle.get(self.vdom.parser()).map_or(None, |node| node.children()) {
                Some(children) => {
                    result.extend(self.get_all_handles(children.top().iter()));
                },
                None => {}
            }
        }
        result
    }

    /// Computes the style for each node in the document and stores them in computed_styles.
    fn compute_styles(&mut self, html_filename: Option<&str>) {
        self.read_styles(html_filename);
        // TODO: Optimize this (vdom.nodes() gives references to nodes but nodes cant hash, and there is no good way to get a node handle from a node. computing styles while collecting the node handles is not possible because...
        // we need to get the node out of the node handle to check if a style matches and to get it's children, but to get the node it is neccessary to borrow the self.vdom.parser() which means we cant borrow self as mutable at the same time, which
        // happen if we would recursively compute the styles for the node's children. Therefore all node handles are currently collected before computing the styles, and then we traverse the whole tree again to compute the styles.
        // There MUST be a better way to do this.
        let all_handles = self.get_all_handles(self.vdom.children());
        for node_handle in all_handles {
            let node = node_handle.get(self.vdom.parser()).unwrap();
            if node.as_tag().is_some() {
                for style in self.all_styles.iter() {
                    if !style.matches(node) {
                        continue;
                    }
                    match self.computed_styles.get_mut(&node_handle) {
                        Some(computed_style) => {
                            computed_style.apply_style(style.clone(), Some(node))
                        },
                        None => {
                            let mut computed_style = crate::css::ComputedStyle::new(crate::css::Selector::complete_selector(node));
                            computed_style.apply_style(style.clone(), Some(node));
                            self.computed_styles.insert(node_handle, computed_style);
                        }
                    }
                }
            }
        }
    }

    /// Creates a new window and parses the given html.
    pub fn new<'a>(video_subsystem: &VideoSubsystem, options: &WindowCreationOptions, html: &'a str, html_filename: Option<&str>) -> Window<'a> {
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
            vdom: tl::parse(html, tl::ParserOptions::default()).unwrap(),
            all_styles: Vec::new(),
            computed_styles: HashMap::new(),
        };
        w.compute_styles(html_filename);
        w
    }

    pub fn draw(&mut self) {
        // TODO optimize this (same problem as in compute_styles)
        let all_handles = self.get_all_handles(self.vdom.children());
        for node_handle in all_handles {
            self.draw_element(&node_handle);
        }
        self.sdl_canvas.present();
    }

    fn draw_element(&mut self, node_handle: &tl::NodeHandle) {
        let node = node_handle.get(self.vdom.parser()).unwrap();
        let style = self.computed_styles.get(node_handle);
        if style.is_some() {
            let style = style.unwrap();
            // Draw background
            let (width, _width_unit) = style.get_value::<f32>("width");
            let (height, _height_unit) = style.get_value::<f32>("height");
            let (background_color, _) = style.get_value::<crate::css::CssColor>("background-color");
            if width.is_some() && height.is_some() && background_color.is_some() {
                let width = width.unwrap();
                let height = height.unwrap();
                let background_color = background_color.unwrap();
                self.sdl_canvas.set_draw_color(background_color.sdl_color);
                self.sdl_canvas.fill_rect(sdl2::rect::Rect::new(0, 0, width as u32, height as u32)).unwrap();
            }
            // Draw text
            let _text = node.inner_text(self.vdom.parser()).to_string();
            // TODO draw text
        }
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
