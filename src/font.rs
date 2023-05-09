use std::collections::HashMap;
use sdl2::render::Canvas;

struct TextCanvasBuilder<'a, Target: sdl2::render::RenderTarget> {
    canvas: &'a mut Canvas<Target>,
    x: f32,
    y: f32,
}

impl<Target : sdl2::render::RenderTarget> TextCanvasBuilder<'_, Target> {
    fn new<'a>(canvas: &'a mut Canvas<Target>) -> TextCanvasBuilder<'a, Target> {
        canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        TextCanvasBuilder {
            canvas,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl<Target : sdl2::render::RenderTarget> ttf_parser::OutlineBuilder for TextCanvasBuilder<'_, Target> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.canvas.draw_line((self.x as i32, self.y as i32), (x as i32, y as i32)).unwrap();
        self.x = x;
        self.y = y;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // Draw quadratic bezier curve
        let x_span = (x - self.x).abs();
        let y_span = (y - self.y).abs();
        let max_span = x_span.max(y_span).ceil() as i32;
        for i in 0..max_span {
            let t = i as f32 / max_span as f32;
            let t_x = (1.0 - t) * (1.0 - t) * self.x + 2.0 * (1.0 - t) * t * x1 + t * t * x;
            let t_y = (1.0 - t) * (1.0 - t) * self.y + 2.0 * (1.0 - t) * t * y1 + t * t * y;
            self.canvas.draw_point((t_x as i32, t_y as i32)).unwrap();
        }
        self.x = x;
        self.y = y;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Draw cubic bezier curve
        let x_span = (x - self.x).abs();
        let y_span = (y - self.y).abs();
        let max_span = x_span.max(y_span).ceil() as i32;
        for i in 0..max_span {
            let t = i as f32 / max_span as f32;
            let t_x = (1.0 - t) * (1.0 - t) * (1.0 - t) * self.x + 3.0 * (1.0 - t) * (1.0 - t) * t * x1 + 3.0 * (1.0 - t) * t * t * x2 + t * t * t * x;
            let t_y = (1.0 - t) * (1.0 - t) * (1.0 - t) * self.y + 3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t * t * y2 + t * t * t * y;
            self.canvas.draw_point((t_x as i32, t_y as i32)).unwrap();
        }
        self.x = x;
        self.y = y;
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
                println!("Loaded face {}: {:?}", index, face);
                font.faces.insert(index, face);
            }
        }
        font
    }

    pub fn face(&self) -> Option<&ttf_parser::Face<'a>> {
        self.faces.get(&self.used_index)
    }

    pub fn render<Target: sdl2::render::RenderTarget>(&self, canvas: & mut Canvas<Target>, text: &str) {
        if let Some(face) = self.face() {
            let mut builder = TextCanvasBuilder::new(canvas);
            for c in text.chars() {
                if let Some(glyph_id) = face.glyph_index(c) {
                    let _bbox = face.outline_glyph(glyph_id, &mut builder);
                    builder.x += face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32;
                }
            }
        }
    }
}