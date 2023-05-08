
use std::collections::HashMap;

use crate::{event::EventReturnCode, font::Font};

pub struct Context<'f, 'ff> {
    sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub fonts: HashMap<String, &'f Font<'ff>>,
}

impl Context<'_, '_> {
    pub fn new<'f, 'ff>() -> Context<'f, 'ff> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        Context {
            sdl,
            video_subsystem,
            fonts: HashMap::new(),
        }
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