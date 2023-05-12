use std::ptr::eq;


#[derive(PartialEq)]
pub enum EventReturnCode {
    Continue,
    Cancel,
    Quit
}

pub struct Event<T>(pub T);

pub trait EventReceiver<T> {
    fn add_listener(&mut self, listener: fn(&Event<T>) -> EventReturnCode);
    fn remove_listener(&mut self, listener: fn(&Event<T>) -> EventReturnCode);
    fn trigger(&self, event: &Event<T>) -> EventReturnCode;
}

pub struct MouseButtonEventData {
    pub button: sdl2::mouse::MouseButton,
    pub clicks: u8,
}

pub struct FingerEventData {
    pub norm_x: f32,
    pub norm_y: f32,
    pub dx: f32,
    pub dy: f32,
    pub touch_id: i64,
    pub finger_id: i64,
    pub norm_pressure: f32,
}

pub struct PointerEventData {
    pub x: i32,
    pub y: i32,
    pub mouse_data: Option<MouseButtonEventData>,
    pub finger_data: Option<FingerEventData>,
    pub timestamp: u32,
}

pub struct PointerDownEvent {
    pub data: PointerEventData,
}
pub struct PointerUpEvent {
    pub data: PointerEventData,
}

pub struct MouseMoveData {
    pub state: sdl2::mouse::MouseState
}

pub struct PointerMoveEvent {
    pub x: i32,
    pub y: i32,
    pub x_rel: i32,
    pub y_rel: i32,
    pub mouse_data: Option<MouseMoveData>,
    pub finger_data: Option<FingerEventData>,
    pub timestamp: u32,
}

pub struct ScrollEvent {
    pub x: i32,
    pub y: i32,
    pub timestamp: u32,
}

pub struct GenericEventReceiver<T> {
    listeners: Vec<fn(&Event<T>) -> EventReturnCode>,
}

impl<T> GenericEventReceiver<T> {
    pub fn new() -> GenericEventReceiver<T> {
        GenericEventReceiver {
            listeners: Vec::new(),
        }
    }   
}

impl<T> EventReceiver<T> for GenericEventReceiver<T> {
    fn add_listener(&mut self, listener: fn(&Event<T>) -> EventReturnCode) {
        self.listeners.push(listener);
    }

    fn remove_listener(&mut self, listener: fn(&Event<T>) -> EventReturnCode) {
        self.listeners.retain(|l| !eq(l, &listener));
    }

    fn trigger(&self, event: &Event<T>) -> EventReturnCode {
        let mut return_code = EventReturnCode::Continue;
        for listener in &self.listeners {
            return_code = listener(event);
            if return_code != EventReturnCode::Continue {
                break;
            }
        }
        return_code
    }
}