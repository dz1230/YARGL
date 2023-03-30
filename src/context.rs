use std::vec::Vec;
use std::rc::Rc;

use crate::event::EventReturnCode;
use crate::window::{Window, WindowCreationOptions};

pub struct Context {
    windows: Vec<Rc<Window>>,
    sdl: sdl2::Sdl,
}

impl Context {
    pub fn new() -> Context {
        let sdl = sdl2::init().unwrap();
        Context {
            windows: Vec::new(),
            sdl: sdl,
        }
    }

    pub fn create_window(&mut self, options: &WindowCreationOptions) -> Rc<Window> {
        let window = Window::new(&self.sdl, options);
        self.windows.push(Rc::new(window));
        self.windows.last().unwrap().clone()
    }

    pub fn poll_events(&self) -> EventReturnCode {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut return_code = EventReturnCode::Continue;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    return_code = EventReturnCode::Quit;
                    break
                },
                _ => {
                    // TODO handle other events
                }
            }
        }
        return_code
    }
}