use glutin;
use gl::types::*;

use std::borrow::Borrow;
use std::{mem, ptr, cmp};
use std::sync::{Arc, Mutex};

// use notify::*;

pub struct Window {
    pub glutin_window: Arc<glutin::Window>,
    pub events_loop: Arc<glutin::EventsLoop>,
    pub gl_program: Option<GLuint>,
}

impl Window {
    pub fn new(name: &str) -> Window {
        let event = glutin::EventsLoop::new();
        let window = glutin::Window::new(&event).unwrap();

        window.set_title(name);

        unsafe { window.make_current() };

        Window {
            glutin_window: Arc::new(window),
            events_loop: Arc::new(event),
            gl_program: None,
        }
    }

    pub fn show(&self) {
        (*self.glutin_window).show();
    }
}
