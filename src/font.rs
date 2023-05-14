use std::collections::{HashMap, HashSet};
use sdl2::render::Canvas;

use crate::layout;

// TODO use rasterized chars instead of outlines

struct TextCanvasBuilder<'a, Target: sdl2::render::RenderTarget> {
    canvas: &'a mut Canvas<Target>,
    x: f32,
    y: f32,
    x0: f32,
    y0: f32,
    scale: f32,
}

impl<Target : sdl2::render::RenderTarget> TextCanvasBuilder<'_, Target> {
    fn new<'a>(x0: f32, y0: f32, scale: f32, canvas: &'a mut Canvas<Target>) -> TextCanvasBuilder<'a, Target> {
        TextCanvasBuilder {
            canvas,
            x: 0.0,
            y: 0.0,
            x0,
            y0,
            scale
        }
    }
}

impl<Target : sdl2::render::RenderTarget> ttf_parser::OutlineBuilder for TextCanvasBuilder<'_, Target> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x * self.scale;
        self.y = y * self.scale;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let scaled_x = x * self.scale;
        let scaled_y = y * self.scale;
        match self.canvas.draw_line(
            ((self.x0 + self.x) as i32, (self.y0 - self.y) as i32), 
            ((self.x0 + scaled_x) as i32, (self.y0 - scaled_y) as i32)
        ) {_ => {}}
        self.x = scaled_x;
        self.y = scaled_y;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let scaled_x1 = x1 * self.scale;
        let scaled_y1 = y1 * self.scale;
        let scaled_x = x * self.scale;
        let scaled_y = y * self.scale;
        // Draw quadratic bezier curve
        let x_span = (scaled_x - self.x).abs();
        let y_span = (scaled_y - self.y).abs();
        let max_span = x_span.max(y_span).ceil() as i32;
        for i in 0..max_span {
            let t = i as f32 / max_span as f32;
            let t_x = (1.0 - t) * (1.0 - t) * self.x + 2.0 * (1.0 - t) * t * scaled_x1 + t * t * scaled_x;
            let t_y = (1.0 - t) * (1.0 - t) * self.y + 2.0 * (1.0 - t) * t * scaled_y1 + t * t * scaled_y;
            match self.canvas.draw_point(((self.x0 + t_x) as i32, (self.y0 - t_y) as i32)) {_ => {}}
        }
        self.x = scaled_x;
        self.y = scaled_y;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let scaled_x1 = x1 * self.scale;
        let scaled_y1 = y1 * self.scale;
        let scaled_x2 = x2 * self.scale;
        let scaled_y2 = y2 * self.scale;
        let scaled_x = x * self.scale;
        let scaled_y = y * self.scale;
        // Draw cubic bezier curve
        let x_span = (scaled_x - self.x).abs();
        let y_span = (scaled_y - self.y).abs();
        let max_span = x_span.max(y_span).ceil() as i32;
        for i in 0..max_span {
            let t = i as f32 / max_span as f32;
            let t_x = (1.0 - t) * (1.0 - t) * (1.0 - t) * self.x + 3.0 * (1.0 - t) * (1.0 - t) * t * scaled_x1 + 3.0 * (1.0 - t) * t * t * scaled_x2 + t * t * t * scaled_x;
            let t_y = (1.0 - t) * (1.0 - t) * (1.0 - t) * self.y + 3.0 * (1.0 - t) * (1.0 - t) * t * scaled_y1 + 3.0 * (1.0 - t) * t * t * scaled_y2 + t * t * t * scaled_y;
            match self.canvas.draw_point(((self.x0 + t_x) as i32, (self.y0 - t_y) as i32)) {_ => {}}
        }
        self.x = scaled_x;
        self.y = scaled_y;
    }

    fn close(&mut self) {
        self.canvas.present();
    }
}

#[derive(Clone)]
pub struct Font<'a> {
    faces: HashMap<u32, ttf_parser::Face<'a>>,
    pub used_index: u32,
}

impl<'a> Font<'a> {
    pub fn new<'b: 'a>(data: &'b Vec<u8>, min: u32, max: u32) -> Font<'a> {
        let mut font = Font {
            faces: HashMap::new(),
            used_index: 0,
        };
        for index in min..max {
            if let Ok(face) = ttf_parser::Face::parse(data, index) {
                font.faces.insert(index, face);
            }
        }
        font
    }

    pub fn face(&self) -> Option<&ttf_parser::Face<'a>> {
        self.faces.get(&self.used_index)
    }

    /// Updates content flow of the given parent layout with the dimensions of the given text. Can break lines at any character.
    pub fn text_layout(&self, text: &str, line_height: i32, parent_layout_mut: &mut layout::NodeLayoutInfo) {
        if let Some(face) = self.face() {
            let units_per_em = face.units_per_em() as f32;
            for c in text.chars() {
                if let Some(glyph) = face.glyph_index(c) {
                    let hor_advance_f = face.glyph_hor_advance(glyph).unwrap_or(0) as f32;
                    let hor_advance = (hor_advance_f / units_per_em * (line_height as f32)) as i32;
                    parent_layout_mut.reverse_flow_inline(hor_advance, line_height);
                }
            }
        }
    }

    // TODO fill shapes (currently only draws outlines)
    /// Dimensions of the given text with this font, using the given line height and max width.
    /// Optionally draws the text on the given canvas.
    /// - text [&str] - Text to measure.
    /// - line_height [i32] - Height of one line of text.
    /// - x0 [i32] - X position of the text.
    /// - max_width [Option<i32>] - Maximum width of the text. Breaks the line if the width is exceeded, but only on characters in the break_on set.
    /// That means the returned width can exceed max_width, if breaking it would result in an empty line.
    /// - break_on [&HashSet<char>] - Characters on which a line break can be insterted due to max_width. If max_width is none, this is ignored.
    /// - draw_on [Option<&mut sdl2::render::Canvas<Target>>] - If given, draws the text on the canvas.
    /// 
    /// Returns a tuple of (width, height, x1, y1). Height is 1 line short.
    pub fn render_text<Target: sdl2::render::RenderTarget>(&self, text: &str, line_height: i32, _font_size: i32, x0: i32, y0: i32, max_width: Option<i32>, break_on: &HashSet<char>, draw_on: Option<&mut sdl2::render::Canvas<Target>>) -> (i32, i32, i32, i32) {
        let mut x = x0;
        let mut y = y0;
        let mut break_x = 0;
        let mut max_x = 0;
        let mut i = 0;
        let mut break_i0 = 0;
        let mut break_i = 0;
        let mut lines: Vec<String> = Vec::new();
        let mut cur_line = String::new();
        if let Some(face) = self.face() {
            let ver_advance_f = face.units_per_em() as f32;
            for c in text.chars() {
                if let Some(glyph_id) = face.glyph_index(c) {
                    if break_on.contains(&c) {
                        break_x = x;
                        break_i = i;
                    }
                    let hor_advance_f = face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32;
                    let hor_advance = (hor_advance_f / ver_advance_f * line_height as f32) as i32;
                    if max_width.is_some() && (x + hor_advance > max_width.unwrap()) {
                        // line break because max width is exceeded
                        if x > max_x {
                            max_x = x;
                        }
                        x = x - break_x;
                        // dont break line if it would result in an empty line (break_x == 0)
                        if break_x > 0 {
                            y += line_height;
                            break_x = 0;
                            let old_line = cur_line.drain(0..(break_i-break_i0)).as_str().to_string();
                            lines.push(old_line);
                            break_i = i;
                            break_i0 = i;
                        }
                    } else if c == '\n' {
                        // line break in text
                        if x > max_x {
                            max_x = x;
                        }
                        x = 0;
                        break_x = 0;
                        break_i0 = i;
                        break_i = i;
                        y += line_height; 
                        lines.push(cur_line.clone());
                        cur_line.clear();
                    } else {
                        x += hor_advance;
                        cur_line.push(c);
                    }
                    i += 1;
                }
            }
            lines.push(cur_line.clone());
            cur_line.clear();
            if x > max_x {
                max_x = x;
            }
        }
        if let Some(canvas) = draw_on {
            if let Some(face) = self.face() {
                let scale = line_height as f32 / face.units_per_em() as f32;
                let mut text_builder = TextCanvasBuilder::new(x0 as f32, (y0 + line_height) as f32, scale, canvas);
                for line in lines {
                    text_builder.x0 = x0 as f32;
                    for c in line.chars() {
                        if let Some(glyph_id) = face.glyph_index(c) {
                            // TODO calculate baseline and y bearing
                            let bearing_x = face.glyph_hor_side_bearing(glyph_id).unwrap_or(0) as f32 * scale;
                            text_builder.x0 += bearing_x;
                            let _bbox = face.outline_glyph(glyph_id, &mut text_builder);
                            text_builder.x0 -= bearing_x;
                            text_builder.x0 += scale * (face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32);
                        }
                    }
                    text_builder.y0 += line_height as f32;
                }
            }
        }
        (max_x, y - y0, x, y)
    }
}