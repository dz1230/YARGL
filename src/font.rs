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

pub struct Font<'a> {
    data: Vec<u8>,
    faces: HashMap<u32, ttf_parser::Face<'a>>,
    pub used_index: u32,
}

// TODO figure out how to tell the borrow checker that the lifetime of the Font is the same as the lifetime of the Face
impl<'a> Font<'a> {
    pub fn new(file: & String, min_index: Option<u32>, max_index: Option<u32>) -> Font<'a> {
        let mut min = min_index.unwrap_or(0);
        let mut max = max_index.unwrap_or(1);
        if min > max {
            let temp = min;
            min = max;
            max = temp;
        }
        let mut font = Font {
            data: std::fs::read(file).unwrap(),
            faces: HashMap::new(),
            used_index: min,
        };
        for index in min..max {
            let face = ttf_parser::Face::parse(&font.data, index).unwrap();
            font.faces.insert(index, face);
        }
        font
    }

    pub fn face(&self) -> &ttf_parser::Face<'a> {
        self.faces.get(&self.used_index).unwrap()
    }

    pub fn render<Target: sdl2::render::RenderTarget>(&self, canvas: & mut Canvas<Target>, text: &str) {
        let face = self.face();
        let mut builder = TextCanvasBuilder::new(canvas);
        for char in text.chars() {
            let glyph_id_option = face.glyph_index(char);
            if glyph_id_option.is_none() {
                continue;
            }
            let glyph_id = glyph_id_option.unwrap();
            face.outline_glyph(glyph_id, &mut builder).unwrap();
            builder.x += face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32;
        }
    }
}