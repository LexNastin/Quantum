use crate::presentation::Presentation;
use crate::presentation::Slide;
use std::vec::Vec;
use std::string::String;

pub enum DisplayRole {
    Main,
    Foldback,
    Other(String),
}

pub trait Display {
    fn render(&mut self, new_slide: &Slide);
}

pub struct Window {
    pub role: DisplayRole,
    pub curr_slide: Slide,
}

impl Display for Window {
    fn render(&mut self, new_slide: &Slide) {
        // TODO: Implement render for different displays
        self.curr_slide = new_slide.clone();
        println!("Display output: {}", self.curr_slide.text);
    }
}

pub struct Output {
    pub displays: Vec<Box<dyn Display>>,
    pub curr_pres: Presentation,
    pub curr_slide: usize,
}

impl Output {
    pub fn init() -> Output {
        let displays: Vec<Box<dyn Display>> = Vec::new();
        let curr_pres = Presentation::blank_presentation();

        Output {
            displays,
            curr_pres,
            curr_slide: 0,
        }
    }

    pub fn new_window(role: DisplayRole) -> Box<Window> {
        Box::new(
            Window {
                role,
                curr_slide: Slide::blank_slide(),
            }
        )
    }

    pub fn new_display(&mut self, mut display: Box<dyn Display>) {
        display.render(
            self.curr_pres.slides
                .get(self.curr_slide)
                .unwrap_or(&Slide::blank_slide())
        );
        self.displays.push(display);
    }

    pub fn load_pres(&mut self, new_pres: Presentation) {
        println!("Loaded presentation: {}", new_pres.title);
        self.render(
            new_pres.slides
                .get(self.curr_slide)
                .unwrap_or(&Slide::blank_slide())
        );
        self.curr_pres = new_pres;
    }

    fn render(&mut self, new_slide: &Slide) {
        for display in self.displays.iter_mut() {
            display.render(new_slide);
        }
    }

    pub fn next(&mut self) {
        println!("Attempting next slide...");
        let length = self.curr_pres.slides.len();
        if self.curr_slide + 1 < length {
            self.curr_slide += 1;
            self.render(
                self.curr_pres.slides.clone()
                    .get(self.curr_slide)
                    .unwrap_or(&Slide::blank_slide())
            );
        }
    }

    pub fn prev(&mut self) {
        println!("Attempting prev slide...");
        if self.curr_slide > 0 {
            self.curr_slide -= 1;
            self.render(
                self.curr_pres.slides.clone()
                    .get(self.curr_slide)
                    .unwrap_or(&Slide::blank_slide())
            );
        }
    }
}
