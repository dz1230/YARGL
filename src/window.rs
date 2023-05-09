use std::{collections::HashMap, rc::Rc};

use tl::VDom;

use crate::{context::Context, css, layout::{NodeLayoutInfo, LayoutValue}};

pub struct WindowCreationOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window<'a, 'f, 'ff> {
    ctx: Rc<Context<'f, 'ff>>,
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    width: u32,
    height: u32,
    vdom: VDom<'a>,
    all_styles: Vec<Rc<css::Style>>,
    computed_styles: HashMap<tl::NodeHandle, css::ComputedStyle>,
    computed_layouts: HashMap<tl::NodeHandle, NodeLayoutInfo>
}

impl Window<'_, '_, '_> {
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
        for style in self.all_styles.iter() {
            for selector in style.selectors.iter() {
                if let Some(nodes) = self.vdom.query_selector(selector.to_string().as_str()) {
                    for node_handle in nodes {
                        let node = node_handle.get(self.vdom.parser()).unwrap();
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
    }

    /// Creates a new window and parses the given html.
    pub fn new<'a, 'f, 'ff>(ctx: Rc<Context<'f, 'ff>>, options: &WindowCreationOptions, html: &'a str, html_filename: Option<&str>) -> Window<'a, 'f, 'ff> {
        let sdl_window = ctx.video_subsystem.window(&options.title, options.width, options.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().unwrap();
        let mut w = Window {
            ctx: ctx.clone(),
            sdl_canvas: canvas,
            width: options.width,
            height: options.height,
            vdom: tl::parse(html, tl::ParserOptions::default()).unwrap(),
            all_styles: Vec::new(),
            computed_styles: HashMap::new(),
            computed_layouts: HashMap::new(),
        };
        w.compute_styles(html_filename);
        w
    }

    // TODO support "auto" values in layout

    /// Calculates layout information for the whole document.
    pub fn layout(&mut self) {
        let all_handles = self.get_all_handles(self.vdom.children());
        for node_handle in all_handles.iter() {
            self.layout_element_top_down(&node_handle, None);
        }
        // let mut flow = sdl2::rect::Point::new(0, 0);
        // for node_handle in all_handles.iter() {
        //     self.layout_position(&mut flow, node_handle);
        // }
        for node_handle in all_handles.iter().rev() {
            self.layout_element_bottom_up(&node_handle);
        }
        // TODO calculate positions
        // TODO calculate masks and overflow
    }

    pub fn draw(&mut self) {
        
        // TODO: Optimize this (vdom.nodes() gives references to nodes but nodes cant hash, and there is no good way to get a node handle from a node. computing styles while collecting the node handles is not possible because...
        // we need to get the node out of the node handle to check if a style matches and to get it's children, but to get the node it is neccessary to borrow the self.vdom.parser() which means we cant borrow self as mutable at the same time, which
        // happen if we would recursively compute the styles for the node's children. Therefore all node handles are currently collected before computing the styles, and then we traverse the whole tree again to compute the styles.
        // There MUST be a better way to do this.
        
        // TODO optimize this (problem see above)
        let all_handles = self.get_all_handles(self.vdom.children());
        for node_handle in all_handles {
            self.draw_element(&node_handle);
        }
        self.sdl_canvas.present();
    }

    /// Calculate the pixel value for the given unit value, if it is either fixed or depends on the parent. Otherwise returns None (i.e. for fit-content).
    /// - Which [usize]: One of layout::X, layout::Y, layout::WIDTH, layout::HEIGHT, layout::FONT_SIZE.
    /// - value [Option<f32>]: Size value.
    /// - unit [Option<css::Unit>]: Unit of the size value.
    /// - parent_handle [Option<&tl::NodeHandle>]: Parent node handle.
    fn calc_size_top_down<const WHICH: usize>(&self, value: Option<f32>, unit: Option<css::Unit>, node_handle: &tl::NodeHandle, parent_handle: Option<&tl::NodeHandle>) -> Option<i32> {
        // magic numbers from https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units
        match unit {
            Some(css::Unit::Px) => Some(value.unwrap_or(0.0) as i32),
            Some(css::Unit::Vw) => Some((value.unwrap_or(0.0) * (self.width as f32) / 100.0) as i32),
            Some(css::Unit::Vh) => Some((value.unwrap_or(0.0) * (self.height as f32) / 100.0) as i32),
            Some(css::Unit::Vmin) => Some((value.unwrap_or(0.0) * (self.width.min(self.height) as f32) / 100.0) as i32),
            Some(css::Unit::Vmax) => Some((value.unwrap_or(0.0) * (self.width.max(self.height) as f32) / 100.0) as i32),
            Some(css::Unit::Cm) => Some((value.unwrap_or(0.0) * 37.79527559055118) as i32),
            Some(css::Unit::Mm) => Some((value.unwrap_or(0.0) * 3.779527559055118) as i32),
            Some(css::Unit::Q) => Some((value.unwrap_or(0.0) * 0.9448818897637795) as i32),
            Some(css::Unit::In) => Some((value.unwrap_or(0.0) * 96.0) as i32),
            Some(css::Unit::Pt) => Some((value.unwrap_or(0.0) * 1.3333333333333333) as i32),
            Some(css::Unit::Pc) => Some((value.unwrap_or(0.0) * 16.0) as i32),
            Some(css::Unit::Percent) => parent_handle
                .map_or(None, |p| self.computed_layouts.get(p))
                .map_or(None, |l| Some(l.get::<{WHICH}>()))
                .map_or(None, |v| v)
                .map_or(None, |v| Some((value.unwrap_or(0.0) * (v as f32) / 100.0) as i32)),
            Some(css::Unit::Em) => match LayoutValue::from(WHICH) {
                LayoutValue::FontSize => parent_handle
                    .map_or(None, |p| self.computed_styles.get(p))
                    .map_or(None, |s| s.get_value::<f32>("font-size").0)
                    .map_or(None, |v| Some((value.unwrap_or(0.0) * v) as i32)),
                _ => self.computed_styles.get(node_handle)
                    .map_or(None, |s| s.get_value::<f32>("font-size").0)
                    .map_or(None, |v| Some((value.unwrap_or(0.0) * v) as i32))
            },
            _ => None
        }
    }

    fn calc_size_bottom_up<const WHICH: usize>(&self, property: &str, node_handle: &tl::NodeHandle) -> Option<i32> {
        self.computed_styles.get(node_handle)
            .map_or(None, |style| style.get_value::<String>(property).0)
            .map_or(None, |identifier| match identifier.as_str() {
                "fit-content" => {
                    return None;
                    // TODO figure out how to calculate content size, with positions
                    // node_handle.get(self.vdom.parser())
                    // .map_or(None, |node| node.children())
                    // .map_or(None, |children| {
                    //     let mut size_value = 0;
                    //     for child_handle in children.top().iter() {
                    //         if let Some(child_layout) = self.computed_layouts.get(child_handle) {
                    //             if let Some(child_size_value) = child_layout.get::<{WHICH}>() {
                    //                 size_value += child_size_value;
                    //             }
                    //         }
                    //     }
                    //     return Some(size_value);
                    // })
                },
                _ => None
            })
    }

    /// Attempts to calculate an element's padding values.
    fn layout_element_padding(&mut self, node_handle: &tl::NodeHandle) {

    }

    /// Determines width and height if they are based on the parent, or fixed.
    fn layout_element_top_down(&mut self, node_handle: &tl::NodeHandle, parent_handle: Option<&tl::NodeHandle>) {
        if !self.computed_layouts.contains_key(node_handle) {
            self.computed_layouts.insert(node_handle.clone(), NodeLayoutInfo::new());
        }
        if let Some(style) = self.computed_styles.get(node_handle) {
            let (display, _) = style.get_value::<css::Display>("display");
            match display {
                Some(css::Display::None) => {
                    if let Some(node_layout) = self.computed_layouts.get_mut(node_handle) {
                        node_layout.set::<{LayoutValue::Width as usize}>(Some(0));
                        node_layout.set::<{LayoutValue::Height as usize}>(Some(0));
                    }
                },
                Some(css::Display::Block) | Some(css::Display::InlineBlock) => {
                    let (width, width_unit) = style.get_value::<f32>("width");
                    let px_width = self.calc_size_top_down::<{LayoutValue::Width as usize}>(width, width_unit, node_handle, parent_handle);
                    let (height, height_unit) = style.get_value::<f32>("height");
                    let px_height = self.calc_size_top_down::<{LayoutValue::Height as usize}>(height, height_unit, node_handle, parent_handle);
                    if let Some(node_layout) = self.computed_layouts.get_mut(node_handle) {
                        node_layout.set::<{LayoutValue::Width as usize}>(px_width);
                        node_layout.set::<{LayoutValue::Width as usize}>(px_height);
                    }
                },
                Some(css::Display::Flex) => {
                    // TODO implement flex layoutg
                },
                Some(css::Display::Grid) => {
                    // TODO implement grid layout
                },
                _ => {}
            }
            
            // TODO check for flex, none, block, inline, inline-block, etc
            
        } else {
            self.computed_layouts.insert(node_handle.clone(), NodeLayoutInfo::new());
        }
    }
    
    fn layout_element_bottom_up(&mut self, node_handle: &tl::NodeHandle) {
        // TODO bottom up box model (fit-content, auto, margin)

    }

    fn layout_position(&mut self, flow: &mut sdl2::rect::Point, node_handle: &tl::NodeHandle) {
        // TODO calculate x and y positions based on document flow + width, height, margins, etc
        
    }

    fn layout_mask(&mut self, node_handle: &tl::NodeHandle) {
        // TODO calculate masks and add scrollbars
    }

    /// Draws a node into this window's canvas. The node has to be part of this window's vdom.
    fn draw_element(&mut self, node_handle: &tl::NodeHandle) {
        // TODO use layout info to draw
        let node = node_handle.get(self.vdom.parser()).unwrap();
        let style = self.computed_styles.get(node_handle);
        if style.is_some() {
            println!("{:?}", node.as_tag().unwrap().name());
            let style = style.unwrap();
            // Draw background
            let (width, _width_unit) = style.get_value::<f32>("width");
            let (height, _height_unit) = style.get_value::<f32>("height");
            let (background_color, _) = style.get_value::<crate::css::CssColor>("background-color");
            if width.is_some() && height.is_some() && background_color.is_some() {
                println!("{:?} {:?} {:?}", width, height, background_color);
                let width = width.unwrap();
                let height = height.unwrap();
                let background_color = background_color.unwrap();
                self.sdl_canvas.set_draw_color(background_color.sdl_color);
                self.sdl_canvas.fill_rect(sdl2::rect::Rect::new(0, 0, width as u32, height as u32)).unwrap();
            }
            // Draw text
            let text = node.inner_text(self.vdom.parser()).to_string();
            let (font_family_opt, _) = style.get_value::<String>("font-family");
            let (font_color_opt, _) = style.get_value::<crate::css::CssColor>("color");
            if font_family_opt.is_some() && font_color_opt.is_some() {
                let font_family = font_family_opt.unwrap();
                let font_color = font_color_opt.unwrap();
                println!("Font: {:?} {:?}", font_family, font_color);
                self.sdl_canvas.set_draw_color(font_color.sdl_color);
                for font_name in font_family.split(',') {
                    let font_name = font_name.trim();
                    // if let Some(font) = self.ctx.get_font(font_name) {
                    if let Some(font) = self.ctx.fonts.get(font_name.to_lowercase().as_str()) {
                        println!("Found font: {:?}", font_name);
                        font.render(&mut self.sdl_canvas, text.as_str());
                        break;
                    }
                }
            }
        }
    }
}
