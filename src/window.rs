use std::{collections::{HashMap, HashSet}, rc::Rc};

use tl::VDom;

use crate::{context::Context, css, layout::{NodeLayoutInfo, LayoutValue}, event, util::{self, DrawingError}};


pub struct WindowCreationOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window<'a, 'f, 'ff, 's> {
    ctx: Rc<Context<'f, 'ff>>,
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // id canvas stores the node id for each pixel in the canvas. Used for hit testing.
    id_canvas: sdl2::render::SurfaceCanvas<'s>,
    width: u32,
    height: u32,
    html_file: String,
    vdom: VDom<'a>,
    computed_styles: HashMap<tl::NodeHandle, css::ComputedStyle>,
    computed_layouts: HashMap<tl::NodeHandle, NodeLayoutInfo>,
    pub pointer_down_events: event::GenericEventReceiver<event::PointerDownEvent>,
    pub pointer_up_events: event::GenericEventReceiver<event::PointerUpEvent>,
    pub pointer_move_events: event::GenericEventReceiver<event::PointerMoveEvent>,
    pub scroll_events: event::GenericEventReceiver<event::ScrollEvent>,
}

impl Window<'_, '_, '_, '_> {
    /// Creates a new window and parses the given html.
    pub fn new<'a, 'f, 'ff, 's>(ctx: Rc<Context<'f, 'ff>>, options: &WindowCreationOptions, html: &'a str, html_filename: Option<&str>) -> Window<'a, 'f, 'ff, 's> {
        let sdl_window = ctx.video_subsystem.window(&options.title, options.width, options.height)
            .position_centered()
            //.resizable()
            .build()
            .unwrap();
        let canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().unwrap();
        let mut w = Window {
            ctx: ctx.clone(),
            sdl_canvas: canvas,
            id_canvas: sdl2::render::SurfaceCanvas::from_surface(sdl2::surface::Surface::new(options.width, options.height, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap()).unwrap(),
            width: options.width,
            height: options.height,
            vdom: tl::parse(html, tl::ParserOptions::default()).unwrap(),
            html_file: html_filename.unwrap_or("").to_string(),
            computed_styles: HashMap::new(),
            computed_layouts: HashMap::new(),
            pointer_down_events: event::GenericEventReceiver::new(),
            pointer_up_events: event::GenericEventReceiver::new(),
            pointer_move_events: event::GenericEventReceiver::new(),
            scroll_events: event::GenericEventReceiver::new(),
        };
        w.compute_styles();
        w
    }

    /// Retrieves the top node at the given cursor position. None if the position is outside the canvas.
    pub fn get_node_at(&self, x: i32, y: i32) -> Option<&tl::Node> {
        match self.get_node_handle_at(x, y) {
            Some(node_handle) => node_handle.get(self.vdom.parser()),
            None => None
        }
    }
    /// Retrieves the handle of the top node at the given cursor position. None if the position is outside the canvas.
    pub fn get_node_handle_at(&self, x: i32, y: i32) -> Option<tl::NodeHandle> {
        if x < 0 || y < 0 || x >= (self.width as i32) || y >= (self.height as i32) {
            return None;
        }
        match self.id_canvas.read_pixels(sdl2::rect::Rect::new(x, y, 1, 1), sdl2::pixels::PixelFormatEnum::RGBA8888)  {
            Ok(pixel) => {
                let id = pixel[0] as u32 + ((pixel[1] as u32) >> 8) + ((pixel[2] as u32) >> 16) + ((pixel[3] as u32) >> 24);
                return Some(tl::NodeHandle::new(id));
            },
            Err(_) => None
        }

    }
    /// Get the underlying sdl window.
    pub fn sdl_window(&self) -> &sdl2::video::Window {
        self.sdl_canvas.window()
    }
    /// Viewport width.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Viewport height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Redraws the whole window.
    pub fn draw(&mut self) {
        // TODO: Optimize this (vdom.nodes() gives references to nodes but nodes cant hash, and there is no good way to get a node handle from a node. computing styles while collecting the node handles is not possible because...
        // we need to get the node out of the node handle to check if a style matches and to get it's children, but to get the node it is neccessary to borrow the self.vdom.parser() which means we cant borrow self as mutable at the same time, which
        // happen if we would recursively compute the styles for the node's children. Therefore all node handles are currently collected before drawing the nodes, and then we traverse the whole tree again to draw the nodes.
        // There MUST be a better way to do this.
        self.id_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0));
        self.id_canvas.clear();
        self.id_canvas.set_blend_mode(sdl2::render::BlendMode::None);
        self.sdl_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF));
        self.sdl_canvas.clear();
        self.sdl_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let all_handles = self.get_all_handles(self.vdom.children(), None);
        for (node_handle, _parent_handle) in all_handles {
            match self.draw_element(&node_handle) {
                Ok(_) => {},
                Err(_err) => {}
            }
        }
        self.sdl_canvas.present();
    }

    /// Re-Calculates layout information for every node in the document.
    pub fn layout(&mut self) {
        self.computed_layouts.clear();
        let all_handles = self.get_all_handles(self.vdom.children(), None);
        for (node_handle, parent_handle) in all_handles.iter() {
            if !self.computed_layouts.contains_key(&node_handle) {
                self.computed_layouts.insert(node_handle.clone(), NodeLayoutInfo::new());
            }
            self.layout_element_font_size(*node_handle, *parent_handle);
            // First attempt to calculate boxes. Cannot calculate content-based boxes yet.
            self.layout_element_top_down(*node_handle, *parent_handle);
            self.layout_element_padding(*node_handle, *parent_handle);
            self.layout_element_border(*node_handle, *parent_handle);
            self.layout_element_margin(*node_handle, *parent_handle);
        }
        for (node_handle, parent_handle) in all_handles.iter().rev() {
            // Second attempt to calculate boxes. 
            // If the content's boxes were calculated in the first attempt 
            // (or a previous call of this attempt, which is why it's reversed), 
            // content-based boxes can now be calculated.
            self.layout_element_bottom_up(*node_handle, *parent_handle);
            self.layout_element_padding(*node_handle, *parent_handle);
            self.layout_element_border(*node_handle, *parent_handle);
            self.layout_element_margin(*node_handle, *parent_handle);
        }
        // Now everything should be calculated, any uncalculated values (due to user error) are set to 0.
        // Therefore we can now calculate which parts are hidden behind others and add scrollbars where neccessary.
        // (Scrollbars appear above the content to avoid layout issues. This behaviour is different to most browsers.)
        for (node_handle, parent_handle) in all_handles.iter() {
            self.layout_mask_top_down(*node_handle, *parent_handle);
            if let Some(layout) = self.computed_layouts.get(node_handle) {
                if let Some(style) = self.computed_styles.get(node_handle) {
                    if let Some(node) = node_handle.get(self.vdom.parser()) {
                        if let Some(tag) = node.as_tag() {
                            println!("{:?}\n {:?}\n {:?}", tag.name(), layout, style.to_string());
                        }
                    }
                }
            }
        }
    }

    /// Applies the given style to all nodes that match the style's selector.
    /// Does not change properties that are already set with a higher specificity.
    /// 
    /// - style: The style to apply.
    pub fn apply_style(&mut self, style: &Rc<css::Style>) {
        for selector in style.selectors.iter() {
            if let Some(nodes) = self.vdom.query_selector(selector.to_string().as_str()) {
                for node_handle in nodes {
                    match self.computed_styles.get_mut(&node_handle) {
                        Some(computed_style) => {
                            computed_style.apply_style(style.clone(), &selector.specificity())
                        },
                        None => {
                            let complete_selector = match node_handle.get(self.vdom.parser()) {
                                Some(node) => css::Selector::complete_selector(node),
                                None => css::Selector::new(None, vec![], None)
                            };
                            let mut computed_style = css::ComputedStyle::new(complete_selector);
                            computed_style.apply_style(style.clone(), &selector.specificity());
                            self.computed_styles.insert(node_handle.clone(), computed_style);
                        }
                    }
                }
            }
        }
    }

    /// Returns parsed css styles from style and link nodes.
    pub fn read_styles(&self) -> Vec<Rc<css::Style>> {
        let mut styles: Vec<Rc<css::Style>> = Vec::new();
        match self.vdom.query_selector("style") {
            Some(style_nodes) => {
                for node_handle in style_nodes {
                    let style_node = node_handle.get(self.vdom.parser()).unwrap();
                    let style_text = style_node.inner_text(self.vdom.parser()).to_string();
                    styles.append(&mut css::parse_css(style_text.as_str()));
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
                    if path_str.starts_with(".") {
                        path = std::path::PathBuf::from(self.html_file.as_str());
                        path.pop();
                        path.push(path_str);
                    } else {
                        path = std::path::PathBuf::from(path_str);
                    }
                    path_str = path.to_str().unwrap();
                    match std::fs::read_to_string(path_str) {
                        Ok(css_text) => {
                            styles.append(&mut css::parse_css(css_text.as_str()));
                        },
                        Err(_) => continue
                    }
                }
            },
            None => {}
        }

        return styles;
    }
    
    /// Returns a vector of all node handles in the document. Sorted in tree order (root, child 1 of root, child 1 of child 1 of root, child 2 of child 1 of root, child 2 of root...).
    /// Call with self.vdom.children() to get all node handles in the document.
    fn get_all_handles<'a, I>(&self, nodes: I, parent: Option<&tl::NodeHandle>) -> Vec<(tl::NodeHandle, Option<tl::NodeHandle>)>
    where I: IntoIterator<Item = &'a tl::NodeHandle> {
        let mut result: Vec<(tl::NodeHandle, Option<tl::NodeHandle>)> = Vec::new();
        for node_handle in nodes {
            result.push((node_handle.clone(), parent.map(|p| p.clone())));
            match node_handle.get(self.vdom.parser()).map_or(None, |node| node.children()) {
                Some(children) => {
                    result.extend(self.get_all_handles(children.top().iter(), Some(node_handle)));
                },
                None => {}
            }
        }
        result
    }

    /// Computes the style for each node in the document and stores them in computed_styles.
    fn compute_styles(&mut self) {
        let styles = self.read_styles();
        for style in styles.iter() {
            self.apply_style(&style.clone());
        }
    }

    /// Fills a quarter of a circle using midpoint circle algorithm.
    fn fill_quarter_circle(&mut self, x0: i32, y0: i32, r: i32, x_direction: i32, y_direction: i32, color: sdl2::pixels::Color, id_color: sdl2::pixels::Color) -> util::DrawingResult {
        util::fill_quarter_circle(x0, y0, r, x_direction, y_direction, color, &mut self.sdl_canvas)?;
        util::fill_quarter_circle(x0, y0, r, x_direction, y_direction, id_color, &mut self.id_canvas)?;
        Ok(util::DrawingSuccess {})
    }

    /// Draws the background of a node onto the window and the id canvas.
    fn draw_background(&mut self, node_handle: tl::NodeHandle) -> util::DrawingResult {
        if let Some(layout) = self.computed_layouts.get(&node_handle) {
            if let Some(style) = self.computed_styles.get(&node_handle) {
                let (display, _) = style.get_value::<css::Display>("display");
                if display.is_some() && display.unwrap() == css::Display::None {
                    return Ok(util::DrawingSuccess {});
                }
                let x = layout.get::<{LayoutValue::X as usize}>().ok_or(DrawingError { msg: "missing x".to_string() })?;
                let y = layout.get::<{LayoutValue::Y as usize}>().ok_or(DrawingError { msg: "missing y".to_string() })?;
                let width = layout.get::<{LayoutValue::Width as usize}>().ok_or(DrawingError { msg: "missing width".to_string() })?;
                let height = layout.get::<{LayoutValue::Height as usize}>().ok_or(DrawingError { msg: "missing height".to_string() })?;
                let padding_top = layout.get::<{LayoutValue::PaddingTop as usize}>().unwrap_or(0);
                let padding_left = layout.get::<{LayoutValue::PaddingLeft as usize}>().unwrap_or(0);
                let padding_bottom = layout.get::<{LayoutValue::PaddingBottom as usize}>().unwrap_or(0);
                let padding_right = layout.get::<{LayoutValue::PaddingRight as usize}>().unwrap_or(0);
                let border_top_width = layout.get::<{LayoutValue::BorderTopWidth as usize}>().unwrap_or(0);
                let border_left_width = layout.get::<{LayoutValue::BorderLeftWidth as usize}>().unwrap_or(0);
                let border_bottom_width = layout.get::<{LayoutValue::BorderBottomWidth as usize}>().unwrap_or(0);
                let border_right_width = layout.get::<{LayoutValue::BorderRightWidth as usize}>().unwrap_or(0);
                let border_top_left_radius = layout.get::<{LayoutValue::BorderTopLeftRadius as usize}>().unwrap_or(0);
                let border_top_right_radius = layout.get::<{LayoutValue::BorderTopRightRadius as usize}>().unwrap_or(0);
                let border_bottom_left_radius = layout.get::<{LayoutValue::BorderBottomLeftRadius as usize}>().unwrap_or(0);
                let border_bottom_right_radius = layout.get::<{LayoutValue::BorderBottomRightRadius as usize}>().unwrap_or(0);

                // TODO mask raw layout values (if overflow is hidden or scroll or auto)
        
                let full_width = width + padding_left + padding_right + border_left_width + border_right_width;
                let full_height = height + padding_top + padding_bottom + border_top_width + border_bottom_width;
                let left_x_offset = border_top_left_radius.max(border_bottom_left_radius);
                let right_x_offset = border_top_right_radius.max(border_bottom_right_radius);
                let inner_width = full_width - left_x_offset - right_x_offset;
                let top_y_offset = border_top_left_radius.max(border_top_right_radius);
                let bottom_y_offset = border_bottom_left_radius.max(border_bottom_right_radius);
                let inner_height = full_height - top_y_offset - bottom_y_offset;

                let id: u32 = node_handle.get_inner();
                let id_color = sdl2::pixels::Color::RGBA((id&0xFF) as u8, ((id&0x00FF) << 8) as u8, ((id&0x0000FF) << 16) as u8, ((id&0x000000FF) << 24) as u8);

                let (background_color_opt, _) = style.get_value::<css::CssColor>("background-color");
                let background_color = background_color_opt.unwrap_or(css::CssColor {sdl_color: sdl2::pixels::Color::RGBA(0xFF, 0xFF, 0xFF, 0x0)});
                self.sdl_canvas.set_draw_color(background_color.sdl_color);
                self.id_canvas.set_draw_color(id_color);

                let inner_rect = sdl2::rect::Rect::new(x + left_x_offset, y + top_y_offset, inner_width as u32, inner_height as u32);
                self.sdl_canvas.fill_rect(inner_rect).map_err(|msg| DrawingError {msg})?;
                self.id_canvas.fill_rect(inner_rect).map_err(|msg| DrawingError {msg})?;
                let left_rect = sdl2::rect::Rect::new(x, y + top_y_offset, left_x_offset as u32, inner_height as u32);
                self.sdl_canvas.fill_rect(left_rect).map_err(|msg| DrawingError {msg})?;
                self.id_canvas.fill_rect(left_rect).map_err(|msg| DrawingError {msg})?;
                let right_rect = sdl2::rect::Rect::new(x + left_x_offset + inner_width as i32, y + top_y_offset, right_x_offset as u32, inner_height as u32);
                self.sdl_canvas.fill_rect(right_rect).map_err(|msg| DrawingError {msg})?;
                self.id_canvas.fill_rect(right_rect).map_err(|msg| DrawingError {msg})?;
                let top_rect = sdl2::rect::Rect::new(x + left_x_offset, y, inner_width as u32, top_y_offset as u32);
                self.sdl_canvas.fill_rect(top_rect).map_err(|msg| DrawingError {msg})?;
                self.id_canvas.fill_rect(top_rect).map_err(|msg| DrawingError {msg})?;
                let bottom_rect = sdl2::rect::Rect::new(x + left_x_offset, y + top_y_offset + inner_height as i32, inner_width as u32, bottom_y_offset as u32);
                self.sdl_canvas.fill_rect(bottom_rect).map_err(|msg| DrawingError {msg})?;
                self.id_canvas.fill_rect(bottom_rect).map_err(|msg| DrawingError {msg})?;

                // TODO figure out how to handle seams between different border colors

                let (border_top_color_opt, _) = style.get_value::<css::CssColor>("border-top-color");
                let border_top_color = border_top_color_opt.unwrap_or(background_color.clone());
                let border_top_rect = sdl2::rect::Rect::new(x + left_x_offset, y, inner_width as u32, border_top_width as u32);
                self.sdl_canvas.set_draw_color(border_top_color.sdl_color);
                self.sdl_canvas.fill_rect(border_top_rect).map_err(|msg| DrawingError {msg})?;
                let (border_left_color_opt, _) = style.get_value::<css::CssColor>("border-left-color");
                let border_left_color = border_left_color_opt.unwrap_or(background_color.clone());
                let border_left_rect = sdl2::rect::Rect::new(x, y + top_y_offset, border_left_width as u32, inner_height as u32);
                self.sdl_canvas.set_draw_color(border_left_color.sdl_color);
                self.sdl_canvas.fill_rect(border_left_rect).map_err(|msg| DrawingError {msg})?;
                let (border_bottom_color_opt, _) = style.get_value::<css::CssColor>("border-bottom-color");
                let border_bottom_color = border_bottom_color_opt.unwrap_or(background_color.clone());
                let border_bottom_rect = sdl2::rect::Rect::new(x + left_x_offset, y + full_height - border_bottom_width as i32, inner_width as u32, border_bottom_width as u32);
                self.sdl_canvas.set_draw_color(border_bottom_color.sdl_color);
                self.sdl_canvas.fill_rect(border_bottom_rect).map_err(|msg| DrawingError {msg})?;
                let (border_right_color_opt, _) = style.get_value::<css::CssColor>("border-right-color");
                let border_right_color = border_right_color_opt.unwrap_or(background_color.clone());
                let border_right_rect = sdl2::rect::Rect::new(x + full_width - border_right_width as i32, y + top_y_offset, border_right_width as u32, inner_height as u32);
                self.sdl_canvas.set_draw_color(border_right_color.sdl_color);
                self.sdl_canvas.fill_rect(border_right_rect).map_err(|msg| DrawingError {msg})?;

                // TODO how to handle decision between left and top border with?
                if border_top_left_radius > 0 {
                    self.fill_quarter_circle(x + left_x_offset, y + top_y_offset, border_top_left_radius, -1, -1, border_top_color.sdl_color, id_color)?;
                    let inner_top_left_radius = if border_top_left_radius > border_top_width { border_top_left_radius - border_top_width } else { 0 };
                    self.fill_quarter_circle(x + left_x_offset, y + top_y_offset, inner_top_left_radius, -1, -1, background_color.sdl_color, id_color)?;
                } else {
                    let top_left_rect = sdl2::rect::Rect::new(x, y, left_x_offset as u32, top_y_offset as u32);
                    self.sdl_canvas.fill_rect(top_left_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(top_left_rect).map_err(|msg| DrawingError {msg})?;
                }
                if border_bottom_left_radius > 0 {
                    self.fill_quarter_circle(x + left_x_offset, y + top_y_offset + inner_height as i32, border_bottom_left_radius, -1, 1, border_bottom_color.sdl_color, id_color)?;
                    let inner_bottom_left_radius = if border_bottom_left_radius > border_bottom_width { border_bottom_left_radius - border_bottom_width } else { 0 };
                    self.fill_quarter_circle(x + left_x_offset, y + top_y_offset + inner_height as i32, inner_bottom_left_radius, -1, 1, background_color.sdl_color, id_color)?;
                } else {
                    let bottom_left_rect = sdl2::rect::Rect::new(x, y + top_y_offset + inner_height as i32, left_x_offset as u32, bottom_y_offset as u32);
                    self.sdl_canvas.fill_rect(bottom_left_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(bottom_left_rect).map_err(|msg| DrawingError {msg})?;
                }
                if border_top_right_radius > 0 {
                    self.fill_quarter_circle(x + left_x_offset + inner_width as i32, y + top_y_offset, border_top_right_radius, 1, -1, border_top_color.sdl_color, id_color)?;
                    let inner_top_right_radius = if border_top_right_radius > border_top_width { border_top_right_radius - border_top_width } else { 0 };
                    self.fill_quarter_circle(x + left_x_offset + inner_width as i32, y + top_y_offset, inner_top_right_radius, 1, -1, background_color.sdl_color, id_color)?;
                } else {
                    let top_right_rect = sdl2::rect::Rect::new(x + left_x_offset + inner_width as i32, y, right_x_offset as u32, top_y_offset as u32);
                    self.sdl_canvas.fill_rect(top_right_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(top_right_rect).map_err(|msg| DrawingError {msg})?;
                    
                }
                if border_bottom_right_radius > 0 {
                    // draw borders in corner
                    self.fill_quarter_circle(x + left_x_offset + inner_width as i32, y + top_y_offset + inner_height as i32, border_bottom_right_radius, 1, 1, border_bottom_color.sdl_color, id_color)?;
                    let inner_bottom_right_radius = if border_bottom_right_radius > border_bottom_width { border_bottom_right_radius - border_bottom_width } else { 0 };
                    // fill corner background
                    self.fill_quarter_circle(x + left_x_offset + inner_width as i32, y + top_y_offset + inner_height as i32, inner_bottom_right_radius, 1, 1, background_color.sdl_color, id_color)?;
                } else {
                    // fill corner background
                    self.sdl_canvas.set_draw_color(background_color.sdl_color);
                    let bottom_right_rect = sdl2::rect::Rect::new(x + left_x_offset + inner_width as i32, y + top_y_offset + inner_height as i32, right_x_offset as u32, bottom_y_offset as u32);
                    self.sdl_canvas.fill_rect(bottom_right_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(bottom_right_rect).map_err(|msg| DrawingError {msg})?;
                    // draw borders in corner
                    self.sdl_canvas.set_draw_color(border_bottom_color.sdl_color);
                    let border_bottom_right_bottom_rect = sdl2::rect::Rect::new(x + left_x_offset + inner_width as i32, y + full_height - border_bottom_width as i32, right_x_offset as u32, border_bottom_width as u32);
                    self.sdl_canvas.fill_rect(border_bottom_right_bottom_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(border_bottom_right_bottom_rect).map_err(|msg| DrawingError {msg})?;
                    let border_bottom_right_right_rect = sdl2::rect::Rect::new(x + full_width - border_right_width as i32, y + top_y_offset + inner_height as i32, border_right_width as u32, bottom_y_offset as u32);
                    self.sdl_canvas.fill_rect(border_bottom_right_right_rect).map_err(|msg| DrawingError {msg})?;
                    self.id_canvas.fill_rect(border_bottom_right_right_rect).map_err(|msg| DrawingError {msg})?;
                }

                return Ok(util::DrawingSuccess {});
            }
        }
        Err(DrawingError {msg: "no layout or style".to_string()})
    }

    /// Draws a node into this window's canvas. The node has to be part of this window's vdom.
    fn draw_element(&mut self, node_handle: &tl::NodeHandle) -> util::DrawingResult {
        self.draw_background(node_handle.clone())?;
        if let Some(node) = node_handle.get(self.vdom.parser()) {
            if let Some(style) = self.computed_styles.get(node_handle) {
                if let Some(layout) = self.computed_layouts.get(node_handle) {
                    // TODO mask layout
                    let x = layout.get::<{LayoutValue::X as usize}>().ok_or(DrawingError{msg: "missing x".to_string()})?;
                    let y = layout.get::<{LayoutValue::Y as usize}>().ok_or(DrawingError{msg: "missing y".to_string()})?;
                    let width = layout.get::<{LayoutValue::Width as usize}>().ok_or(DrawingError{msg: "missing width".to_string()})?;
                    //let height = layout.get::<{LayoutValue::Height as usize}>().ok_or(DrawingError{msg: "missing height".to_string()})?;
                    let font_size = layout.get::<{LayoutValue::FontSize as usize}>().ok_or(DrawingError{msg: "missing font size".to_string()})?;
                    // Draw text
                    let text = node.inner_text(self.vdom.parser()).to_string();
                    let (font_family_opt, _) = style.get_value::<String>("font-family");
                    let (font_color_opt, _) = style.get_value::<css::CssColor>("color");
                    if font_family_opt.is_some() && font_color_opt.is_some() {
                        println!("Drawing text");
                        let font_family = font_family_opt.unwrap();
                        let font_color = font_color_opt.unwrap();
                        println!("Font: {:?} {:?}", font_family, font_color);
                        self.sdl_canvas.set_draw_color(font_color.sdl_color);
                        for font_name in font_family.split(',') {
                            let font_name = font_name.trim();
                            if let Some(font) = self.ctx.fonts.get(font_name.to_lowercase().as_str()) {
                                println!("Found font: {:?}", font_name);
                                let breakable = HashSet::from_iter(vec![' ', '\n', '\t', '\r', '\u{00A0}'].into_iter());
                                font.text_dimensions(text.as_str(), font_size, font_size, x, y, Some(width), &breakable, Some(&mut self.sdl_canvas));
                                break;
                            }
                        }
                    }
                    return Ok(util::DrawingSuccess {});
                }
            }
        }
        Err(DrawingError{msg: "failed to draw element".to_string()})
    }

    /// Calculate the pixel value for the given unit value, if it is either fixed or depends on the parent. Otherwise returns None (i.e. for fit-content).
    /// - Which [usize]: One of [LayoutValue].
    /// - value [Option< f32 >]: Size value.
    /// - unit [Option< Unit >]: Unit of the size value.
    /// - parent_handle [Option<&tl::NodeHandle>]: Parent node handle.
    /// 
    /// Returns [Option< i32 >]: Pixel value.
    fn calc_size_top_down<const WHICH: usize>(&self, value: Option<f32>, unit: Option<css::Unit>, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) -> Option<i32> {
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
            Some(css::Unit::Percent) => match LayoutValue::from(WHICH) {
                // padding, border, margin are calculated relative to the element's width
                LayoutValue::PaddingBottom | LayoutValue::PaddingLeft | LayoutValue::PaddingRight | LayoutValue::PaddingTop |
                LayoutValue::BorderTopWidth | LayoutValue::BorderLeftWidth | LayoutValue::BorderBottomWidth | LayoutValue::BorderRightWidth |
                LayoutValue::BorderBottomLeftRadius | LayoutValue::BorderBottomRightRadius | LayoutValue::BorderTopLeftRadius | LayoutValue::BorderTopRightRadius
                 => self.computed_layouts.get(&node_handle)
                    .map_or(None, |l| l.get::<{LayoutValue::Width as usize}>())
                    .map_or(None, |w| Some((value.unwrap_or(0.0) * (w as f32) / 100.0) as i32)),
                // other relative properties are calculated relative to the same property on the parent
                _ => match parent_handle {
                    Some(p) => self.computed_layouts.get(&p)
                    .map_or(None, |l| Some(l.get::<{WHICH}>()))
                    .map_or(None, |v| v)
                    .map_or(None, |v| Some((value.unwrap_or(0.0) * (v as f32) / 100.0) as i32)),
                    // for root elements, the parent is the viewport
                    None => match LayoutValue::from(WHICH) {
                        LayoutValue::Height => Some((value.unwrap_or(0.0) * (self.height as f32) / 100.0) as i32),
                        _ => Some((value.unwrap_or(0.0) * (self.width as f32) / 100.0) as i32)
                    }
                }
            },
            Some(css::Unit::Em) => match LayoutValue::from(WHICH) {
                // font-size in em is calculated relative to the parent's font-size
                LayoutValue::FontSize => parent_handle
                    .map_or(None, |p| self.computed_layouts.get(&p))
                    .map_or(None, |l| l.get::<{LayoutValue::FontSize as usize}>())
                    .map_or(None, |v| Some((value.unwrap_or(0.0) * (v as f32)) as i32)),
                // other em values are calculated relative to the element's font-size
                _ => self.computed_layouts.get(&node_handle)
                    .map_or(None, |l: &NodeLayoutInfo| l.get::<{LayoutValue::FontSize as usize}>())
                    .map_or(None, |v| Some((value.unwrap_or(0.0) * (v as f32)) as i32))
            },
            _ => None
        }
    }

    /// Sets a layout value using self.calc_size_top_down. If the value is already set, it will not be overwritten.
    /// - property [&str]: CSS property name.
    /// - node_handle [&tl::NodeHandle]: Node handle.
    /// - parent_handle [Option<&tl::NodeHandle>]: Parent node handle.
    fn set_size_value_top_down<const WHICH: usize>(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        if self.computed_layouts.get(&node_handle).map_or(None, |l| l.get::<{WHICH}>()).is_some() {
            return;
        }
        if let Some(style) = self.computed_styles.get(&node_handle) {
            let (value, unit) = style.get_value::<f32>(LayoutValue::from(WHICH).to_string().as_str());
            let value = self.calc_size_top_down::<{WHICH}>(value, unit, node_handle, parent_handle);
            self.computed_layouts.get_mut(&node_handle).map(|l| l.set::<{WHICH}>(value));
        }
    }

    /// Attempts to calculate an element's font size.
    fn layout_element_font_size(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        self.set_size_value_top_down::<{LayoutValue::FontSize as usize}>(node_handle, parent_handle);
    }

    /// Attempts to calculate an element's padding values.
    fn layout_element_padding(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        self.set_size_value_top_down::<{LayoutValue::PaddingTop as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::PaddingRight as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::PaddingBottom as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::PaddingLeft as usize}>(node_handle, parent_handle);
    }

    /// Attemps to calculate an element's border widths.
    fn layout_element_border(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        self.set_size_value_top_down::<{LayoutValue::BorderTopWidth as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderRightWidth as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderBottomWidth as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderLeftWidth as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderTopLeftRadius as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderTopRightRadius as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderBottomRightRadius as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::BorderBottomLeftRadius as usize}>(node_handle, parent_handle);
    }

    /// Attempts to calculate an element's margin values.
    fn layout_element_margin(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        self.set_size_value_top_down::<{LayoutValue::MarginTop as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::MarginRight as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::MarginBottom as usize}>(node_handle, parent_handle);
        self.set_size_value_top_down::<{LayoutValue::MarginLeft as usize}>(node_handle, parent_handle);
    }

    /// Determines width and height if they are based on the parent, or fixed.
    fn layout_element_top_down(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        if let Some(style) = self.computed_styles.get(&node_handle) {
            // println!("layout_element_top_down: {:?} style {:?}", node_handle, style);
            let (display, _) = style.get_value::<css::Display>("display");
            match display {
                Some(css::Display::None) => {
                    if let Some(node_layout) = self.computed_layouts.get_mut(&node_handle) {
                        node_layout.set::<{LayoutValue::Width as usize}>(Some(0));
                        node_layout.set::<{LayoutValue::Height as usize}>(Some(0));
                    }
                },
                Some(css::Display::Block) | Some(css::Display::InlineBlock) => {
                    let (width, width_unit) = style.get_value::<f32>("width");
                    let px_width = self.calc_size_top_down::<{LayoutValue::Width as usize}>(width, width_unit, node_handle, parent_handle);
                    let (height, height_unit) = style.get_value::<f32>("height");
                    let px_height = self.calc_size_top_down::<{LayoutValue::Height as usize}>(height, height_unit, node_handle, parent_handle);
                    if let Some(node_layout) = self.computed_layouts.get_mut(&node_handle) {
                        node_layout.set::<{LayoutValue::Width as usize}>(px_width);
                        node_layout.set::<{LayoutValue::Height as usize}>(px_height);
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
        } else {
            self.computed_layouts.insert(node_handle.clone(), NodeLayoutInfo::new());
        }
    }
     
    /// Determines width and height if they are based on the content. 
    fn layout_element_bottom_up(&mut self, node_handle: tl::NodeHandle, parent_handle: Option<tl::NodeHandle>) {
        // TODO use paddings, margins, borders in content size calculation
        if let Some(node) = node_handle.get(self.vdom.parser()) {
            if let Some(style) = self.computed_styles.get(&node_handle) {
                let (display, _) = style.get_value::<css::Display>("display");
                match display {
                    Some(css::Display::Inline) => {
                        if let Some(layout) = self.computed_layouts.get(&node_handle) {
                            let text_content = (match node {
                                tl::Node::Tag(tag) => tag.inner_text(self.vdom.parser()).to_string(),
                                tl::Node::Raw(text) => text.try_as_utf8_str().map_or("".to_string(), |s| s.to_string()),
                                _ => "".to_string()
                            }).trim().to_string();
                            if text_content.len() > 0 {
                                let (font_family_opt, _) = style.get_value::<String>("font-family");
                                if let Some(font_family) = font_family_opt {
                                    if let Some(font_size) = layout.get::<{LayoutValue::FontSize as usize}>() {
                                        let mut parent_x = 0;
                                        let mut parent_y = 0;
                                        let mut parent_width: Option<i32> = None;
                                        if let Some(parent_layout) = parent_handle.map_or(None, |p| self.computed_layouts.get(&p)) {
                                            parent_x = parent_layout.get::<{LayoutValue::ContentX as usize}>().unwrap_or(0);
                                            parent_y = parent_layout.get::<{LayoutValue::ContentY as usize}>().unwrap_or(0);
                                            parent_width = parent_layout.get::<{LayoutValue::Width as usize}>();
                                        }
                                        if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                            layout_mut.set::<{LayoutValue::X as usize}>(Some(parent_x));
                                            layout_mut.set::<{LayoutValue::Y as usize}>(Some(parent_y));
                                        }
                                        for font_name in font_family.split(',') {
                                            let font_name = font_name.trim();
                                            if let Some(font) = self.ctx.fonts.get(font_name.to_lowercase().as_str()) {
                                                let breakable = HashSet::from_iter(vec![' ', '\n', '\t', '\r', '\u{00A0}'].into_iter());
                                                let (text_width, text_height, new_content_x, _) = font.text_dimensions::<sdl2::video::Window>(text_content.as_str(), font_size, font_size, -parent_x, 0, parent_width, &breakable, None);
                                                if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                                    layout_mut.set::<{LayoutValue::Width as usize}>(Some(text_width));
                                                    layout_mut.set::<{LayoutValue::Height as usize}>(Some(text_height));
                                                }
                                                if let Some(parent_layout_mut) = parent_handle.map_or(None, |p| self.computed_layouts.get_mut(&p)) {
                                                    // content x and y is negative because the last child is computed first. In the end the values are offset by contentWidth and contentHeight which should get to 0,0
                                                    parent_layout_mut.set::<{LayoutValue::ContentX as usize}>(Some(-new_content_x));
                                                    parent_layout_mut.set::<{LayoutValue::ContentY as usize}>(Some(parent_y - text_height));
                                                    match parent_layout_mut.get::<{LayoutValue::ContentWidth as usize}>() {
                                                        Some(parent_content_width) => {
                                                            if parent_content_width < text_width {
                                                                parent_layout_mut.set::<{LayoutValue::ContentWidth as usize}>(Some(text_width));
                                                            }
                                                        },
                                                        None => {
                                                            parent_layout_mut.set::<{LayoutValue::ContentWidth as usize}>(Some(text_width));
                                                        }
                                                    }
                                                    match parent_layout_mut.get::<{LayoutValue::ContentHeight as usize}>() {
                                                        Some(parent_content_height) => {
                                                            parent_layout_mut.set::<{LayoutValue::ContentHeight as usize}>(Some(parent_content_height + text_height));
                                                        },
                                                        None => {
                                                            parent_layout_mut.set::<{LayoutValue::ContentHeight as usize}>(Some(text_height));
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(css::Display::Block | css::Display::InlineBlock) => {
                        // calculates width and height based on content, if not set already
                        if let Some(layout) = self.computed_layouts.get(&node_handle) {
                            if layout.get::<{LayoutValue::Width as usize}>().is_none() {
                                if let Some(content_width) = layout.get::<{LayoutValue::ContentWidth as usize}>() {
                                    if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                        layout_mut.set::<{LayoutValue::Width as usize}>(Some(content_width));
                                    }
                                }
                            }
                        }
                        if let Some(layout) = self.computed_layouts.get(&node_handle) {
                            if layout.get::<{LayoutValue::Height as usize}>().is_none() {
                                if let Some(content_height) = layout.get::<{LayoutValue::ContentHeight as usize}>() {
                                    if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                        layout_mut.set::<{LayoutValue::Height as usize}>(Some(content_height));
                                    }
                                }
                            }
                        }
                        // get x and y from parent, and update parents contentX and contentY and contentWidth and contentHeight
                        let mut content_x = 0;
                        if display.unwrap() == css::Display::Block {
                            if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                // block elements break lines, therefore always have X = 0
                                layout_mut.set::<{LayoutValue::X as usize}>(Some(0));
                            }
                        } else {
                            if let Some(parent_layout) = parent_handle.map_or(None, |p| self.computed_layouts.get(&p)) {
                                content_x = parent_layout.get::<{LayoutValue::ContentX as usize}>().unwrap_or(0);
                            }
                            if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                                layout_mut.set::<{LayoutValue::X as usize}>(Some(content_x));
                            }
                        }
                        let mut content_y = 0;
                        if let Some(parent_layout) = parent_handle.map_or(None, |p| self.computed_layouts.get(&p)) {
                            content_y = parent_layout.get::<{LayoutValue::ContentY as usize}>().unwrap_or(0);
                        }
                        if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
                            layout_mut.set::<{LayoutValue::Y as usize}>(Some(content_y));
                        }
                        let mut own_width = 0;
                        let mut own_height = 0;
                        if let Some(layout) = self.computed_layouts.get(&node_handle) {
                            own_width = layout.get::<{LayoutValue::Width as usize}>().unwrap_or(0);
                            own_height = layout.get::<{LayoutValue::Height as usize}>().unwrap_or(0);
                        }
                        if let Some(parent_layout_mut) = parent_handle.map_or(None, |p| self.computed_layouts.get_mut(&p)) {
                            // content x and y is negative because the last child is computed first. In the end the values are offset by contentWidth and contentHeight which should get to 0,0
                            parent_layout_mut.set::<{LayoutValue::ContentX as usize}>(Some(content_x -own_width));
                            parent_layout_mut.set::<{LayoutValue::ContentY as usize}>(Some(content_y - own_height));
                            match parent_layout_mut.get::<{LayoutValue::ContentWidth as usize}>() {
                                Some(parent_content_width) => {
                                    if parent_content_width < own_width {
                                        parent_layout_mut.set::<{LayoutValue::ContentWidth as usize}>(Some(own_width));
                                    }
                                },
                                None => {
                                    parent_layout_mut.set::<{LayoutValue::ContentWidth as usize}>(Some(own_width));
                                }
                            }
                            match parent_layout_mut.get::<{LayoutValue::ContentHeight as usize}>() {
                                Some(parent_content_height) => {
                                    parent_layout_mut.set::<{LayoutValue::ContentHeight as usize}>(Some(parent_content_height + own_height));
                                },
                                None => {
                                    parent_layout_mut.set::<{LayoutValue::ContentHeight as usize}>(Some(own_height));
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    /// Calculates MaskedX, MaskedY, MaskedWidth and MaskedHeight for the given node. Converts X, Y from relative to absolute.
    fn layout_mask_top_down(&mut self, _node_handle: tl::NodeHandle, _parent_handle: Option<tl::NodeHandle>) {
        // TODO doesnt seem to work properly at all, redo all of this
        // let mut parent_content_width = 0;
        // let mut parent_content_height = 0;
        // let mut parent_x = 0;
        // let mut parent_y = 0;
        // let mut parent_width = 0;
        // let mut parent_height = 0;
        // if let Some(parent_layout) = parent_handle.map_or(None, |p| self.computed_layouts.get(&p)) {
        //     parent_content_width = parent_layout.get::<{LayoutValue::ContentWidth as usize}>().unwrap_or(0);
        //     parent_content_height = parent_layout.get::<{LayoutValue::ContentHeight as usize}>().unwrap_or(0);
        //     parent_x = parent_layout.get::<{LayoutValue::X as usize}>().unwrap_or(0);
        //     parent_y = parent_layout.get::<{LayoutValue::Y as usize}>().unwrap_or(0);
        //     parent_width = parent_layout.get::<{LayoutValue::Width as usize}>().unwrap_or(0);
        //     parent_height = parent_layout.get::<{LayoutValue::Height as usize}>().unwrap_or(0);
        // }
        // let mut own_x = 0;
        // let mut own_y = 0;
        // let mut own_width = 0;
        // let mut own_height = 0;
        // if let Some(layout) = self.computed_layouts.get(&node_handle) {
        //     own_x = layout.get::<{LayoutValue::X as usize}>().unwrap_or(0);
        //     own_y = layout.get::<{LayoutValue::Y as usize}>().unwrap_or(0);
        //     own_width = layout.get::<{LayoutValue::Width as usize}>().unwrap_or(0);
        //     own_height = layout.get::<{LayoutValue::Height as usize}>().unwrap_or(0);
        // }
        // if let Some(layout_mut) = self.computed_layouts.get_mut(&node_handle) {
        //     // TODO adding content_width here might result in a wrong value for siblings of block elements, which reset the content X and therefore the X of this element
        //     // own_x += parent_x + parent_content_width;
        //     // own_y += parent_y + parent_content_height;
        //     // layout_mut.set::<{LayoutValue::X as usize}>(Some(own_x));
        //     // layout_mut.set::<{LayoutValue::Y as usize}>(Some(own_y));
        //     // // horizontal mask
        //     // if own_x < parent_x && own_x + own_width > parent_x + parent_width {
        //     //     // left and right overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedWidth as usize}>(Some(parent_width));
        //     //     layout_mut.set::<{LayoutValue::MaskedX as usize}>(Some(parent_x));
        //     // } else if own_x < parent_x && own_x + own_width > parent_x {
        //     //     // left overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedWidth as usize}>(Some(own_x + own_width - parent_x));
        //     //     layout_mut.set::<{LayoutValue::MaskedX as usize}>(Some(parent_x));
        //     // } else if own_x >= parent_x && own_x + own_width > parent_x + parent_width {
        //     //     // right overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedWidth as usize}>(Some(parent_x + parent_width - own_x));
        //     //     layout_mut.set::<{LayoutValue::MaskedX as usize}>(Some(own_x));
        //     // } else if own_x >= parent_x && own_x + own_width <= parent_x + parent_width {
        //     //     // element inside parent (horizontally)
        //     //     layout_mut.set::<{LayoutValue::MaskedWidth as usize}>(Some(own_width));
        //     //     layout_mut.set::<{LayoutValue::MaskedX as usize}>(Some(own_x));
        //     // } else {
        //     //     // element outside parent (horizontally)
        //     //     layout_mut.set::<{LayoutValue::MaskedWidth as usize}>(Some(0));
        //     //     layout_mut.set::<{LayoutValue::MaskedX as usize}>(Some(0));
        //     // }
        //     // // vertical mask
        //     // if own_y < parent_y && own_y + own_height > parent_y + parent_height {
        //     //     // top and bottom overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedHeight as usize}>(Some(parent_height));
        //     //     layout_mut.set::<{LayoutValue::MaskedY as usize}>(Some(parent_y));
        //     // } else if own_y < parent_y && own_y + own_height > parent_y {
        //     //     // top overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedHeight as usize}>(Some(own_y + own_height - parent_y));
        //     //     layout_mut.set::<{LayoutValue::MaskedY as usize}>(Some(parent_y));
        //     // } else if own_y >= parent_y && own_y + own_height > parent_y + parent_height {
        //     //     // bottom overflow
        //     //     layout_mut.set::<{LayoutValue::MaskedHeight as usize}>(Some(parent_y + parent_height - own_y));
        //     //     layout_mut.set::<{LayoutValue::MaskedY as usize}>(Some(own_y));
        //     // } else if own_y >= parent_y && own_y + own_height <= parent_y + parent_height {
        //     //     // element inside parent (vertically)
        //     //     layout_mut.set::<{LayoutValue::MaskedHeight as usize}>(Some(own_height));
        //     //     layout_mut.set::<{LayoutValue::MaskedY as usize}>(Some(own_y));
        //     // } else {
        //     //     // element outside parent (vertically)
        //     //     layout_mut.set::<{LayoutValue::MaskedHeight as usize}>(Some(0));
        //     //     layout_mut.set::<{LayoutValue::MaskedY as usize}>(Some(0));
        //     // }
        // }
    }
}
