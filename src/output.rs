use crate::presentation::Presentation;
use crate::presentation::Slide;
use std::vec::Vec;
use std::string::String;

pub enum DisplayMode {
    Window,
    Virtual,
}

pub enum DisplayRole {
    Main,
    Foldback,
    Other(String),
}

pub struct Display {
    pub mode: DisplayMode,
    pub role: DisplayRole,
    current_slide: Slide,
}

impl Display {
    pub fn render(&self, new_slide: &Slide) {
        // TODO: Implement render for different displays
        match self.mode {
            DisplayMode::Window => {},
            DisplayMode::Virtual => {},
        }
    }
}

pub struct Output {
    pub displays: Vec<Display>,
    pub curr_pres: Presentation,
}

impl Output {
    pub fn init() -> Output {
        let displays: Vec<Display> = Vec::new();
        let curr_pres = Presentation::blank_presentation();

        Output {
            displays,
            curr_pres,
        }
    }
    pub fn render(&self, new_slide: Slide) {
        for display in self.displays.iter() {
            display.render(&new_slide);
        }
    }
}
