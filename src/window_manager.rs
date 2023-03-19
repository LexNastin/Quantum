use nannou::prelude::*;
use std::collections::HashMap;
use crate::presentation::Slide;

pub struct Window {
    pub title: String,
    pub position: (i32, i32),
    pub size: Option<(i32, i32)>, // if fullscreen, this won't have a value
}

// TODO: make "new" functions actually create a window
// TODO: implement startup window (as "new" function)
// TODO: remove the following allow
#[allow(unused_variables)]
impl Window {
    pub fn new_windowed(title: String, size: (i32, i32)) -> Self {
        // TODO: Find screen centre and use that
        let position = (0, 0);
        Self {
            title,
            position,
            size: Some(size),
        }
    }

    pub fn new_fullscreen(title: String, position: (i32, i32)) -> Self {
        Self {
            title,
            position,
            size: None,
        }
    }

    pub fn render_slide(&self, curr_slide: &Slide, new_slide: &Slide) -> bool {
        // TODO: implement rendering slides
        true
    }

    pub fn close(&self, title: String) -> bool {
        // TODO: implement closing
        true
    }
}

pub struct WindowManager {
    windows: HashMap<String, Window>,
}

impl WindowManager {
    pub fn init() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn add_window(&mut self, name: String, window: Window) {
        self.windows.insert(name, window);
    }

    pub fn get_window(&self, name: &String) -> Option<&Window> {
        self.windows.get(name)
    }
}
