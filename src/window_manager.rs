use nannou::prelude::*;
use std::collections::HashMap;

pub struct Window {
    pub title: String,
    pub position: (i32, i32),
    pub size: Option<(i32, i32)>, // if fullscreen, this won't have a value
}

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
