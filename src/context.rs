
use crate::event::EventReturnCode;

pub struct Context {
    sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
}

impl Context {
    pub fn new() -> Context {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        Context {
            sdl,
            video_subsystem
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