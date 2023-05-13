
#[derive(Debug, PartialEq, Clone)]
pub struct DrawingError {
    pub msg: String
}
#[derive(Debug)]
pub struct DrawingSuccess;

pub type DrawingResult = Result<DrawingSuccess, DrawingError>;

/// Fills a quarter of a circle using the midpoint circle algorithm.
/// 
/// - x0, y0: Center of the circle
/// - r: Radius of the circle
/// - x_direction: 1 or -1, depending on which direction the circle should be drawn in the x axis
/// - y_direction: 1 or -1, depending on which direction the circle should be drawn in the y axis
/// - color: Color of the circle
/// - canvas: Canvas to draw on
/// 
/// Returns Ok(DrawingSuccess) if the circle was drawn successfully, Err(DrawingError) otherwise.
pub fn fill_quarter_circle<Target: sdl2::render::RenderTarget>(x0: i32, y0: i32, r: i32, x_direction: i32, y_direction: i32, color: sdl2::pixels::Color, canvas: &mut sdl2::render::Canvas<Target>) -> DrawingResult {
    if r <= 0 {
        return Err(DrawingError {msg: "Radius must be greater than 0".to_string()});
    }
    let mut x = r;
    let mut y = 0;
    let mut err = 0;
    canvas.set_draw_color(color);
    while x >= y {
        canvas.draw_line((x0, y0), (x0 + x * x_direction, y0 + y * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 + y * x_direction, y0 + x * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 - y * x_direction, y0 + x * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 - x * x_direction, y0 + y * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 - x * x_direction, y0 - y * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 - y * x_direction, y0 - x * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 + y * x_direction, y0 - x * y_direction)).map_err(|msg| DrawingError {msg})?;
        canvas.draw_line((x0, y0), (x0 + x * x_direction, y0 - y * y_direction)).map_err(|msg| DrawingError {msg})?;
        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
    Ok(DrawingSuccess {})
}
