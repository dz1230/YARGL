
use std::collections::HashMap;

use crate::{event::{EventReturnCode, EventReceiver, self}, font::Font};

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

    pub fn poll_events(&self, windows: &Vec<crate::window::Window>) -> EventReturnCode {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut return_code = EventReturnCode::Continue;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    return_code = EventReturnCode::Quit;
                    break
                },
                sdl2::event::Event::MouseButtonDown { timestamp, window_id, which: _, mouse_btn, clicks, x, y } => {
                    for window in windows {
                        if window.sdl_window().id() == window_id {
                            window.pointer_down_events.trigger(&event::Event(event::PointerDownEvent { 
                                data: event::PointerEventData {
                                    x,
                                    y,
                                    mouse_data: Some(event::MouseButtonEventData {
                                        button: mouse_btn,
                                        clicks,
                                    }),
                                    finger_data: None,
                                    timestamp,
                                }
                            }), window);
                            break;
                        }
                    }
                },
                // TODO support touch input
                // sdl2::event::Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {
                    
                // },
                sdl2::event::Event::MouseButtonUp { timestamp, window_id, which: _, mouse_btn, clicks, x, y } => {
                    for window in windows {
                        if window.sdl_window().id() == window_id {
                            return_code = window.pointer_up_events.trigger(&event::Event(event::PointerUpEvent { 
                                data: event::PointerEventData {
                                    x,
                                    y,
                                    mouse_data: Some(event::MouseButtonEventData {
                                        button: mouse_btn,
                                        clicks,
                                    }),
                                    finger_data: None,
                                    timestamp,
                                }
                            }), window);
                            break;
                        }
                    }
                },
                // sdl2::event::Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {

                // },
                sdl2::event::Event::MouseMotion { timestamp, window_id, which: _, mousestate, x, y, xrel, yrel } => {
                    for window in windows {
                        if window.sdl_window().id() == window_id {
                            return_code = window.pointer_move_events.trigger(&event::Event(event::PointerMoveEvent {
                                x,
                                y,
                                x_rel: xrel,
                                y_rel: yrel,
                                mouse_data: Some(event::MouseMoveData {
                                    state: mousestate,
                                }),
                                finger_data: None,
                                timestamp,
                            }), window);
                            break;
                        }
                    }
                },
                // sdl2::event::Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {

                // },
                sdl2::event::Event::MouseWheel { timestamp, window_id, which: _, x, y, direction } => {
                    for window in windows {
                        if window.sdl_window().id() == window_id {
                            return_code = window.scroll_events.trigger(&event::Event(event::ScrollEvent {
                                x: if direction == sdl2::mouse::MouseWheelDirection::Normal {x} else {-x},
                                y: if direction == sdl2::mouse::MouseWheelDirection::Normal {y} else {-y},
                                timestamp,
                            }), window);
                            break;
                        }
                    }
                },
                // sdl2::event::Event::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => {

                // },
                // TODO support keyboard input
                // sdl2::event::Event::TextInput { timestamp, window_id, text } => {

                // },
                // sdl2::event::Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {

                // },
                // sdl2::event::Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => {

                // },
                _ => {
                    // TODO handle other events
                }
            }
        }
        return_code
    }
}