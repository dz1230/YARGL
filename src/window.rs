
pub struct WindowCreationOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Window {
    sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(sdl: &sdl2::Sdl, options: &WindowCreationOptions) -> Window {
        let video_subsystem = sdl.video().unwrap();
        let sdl_window = video_subsystem.window(&options.title, options.width, options.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().unwrap();
        Window {
            sdl_canvas: canvas,
        }
    }
}