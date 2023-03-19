use crate::presentation::Presentation;
use crate::presentation::Slide;
use crate::window_manager;
use std::vec::Vec;
use std::string::String;
use std::cell::RefCell;
use std::rc::Rc;

pub enum DisplayRole {
    Main,
    Foldback,
    Other(String),
}

pub trait Display {
    fn render(&mut self, new_slide: &Slide);
}

pub struct Window {
    pub window_name: String,
    window_manager: Rc<RefCell<window_manager::WindowManager>>,
    pub role: DisplayRole,
    pub curr_slide: Slide,
}

impl Display for Window {
    fn render(&mut self, new_slide: &Slide) {
        let window = self.window_manager.borrow().get_window(&self.window_name);
        self.curr_slide = new_slide.clone();
        println!("Display output: {}", self.curr_slide.text);
    }
}

pub struct Output {
    pub displays: Vec<Rc<RefCell<dyn Display>>>,
    pub curr_pres: Presentation,
    pub curr_slide: usize,
    pub window_manager: Rc<RefCell<window_manager::WindowManager>>,
}

impl Output {
    pub fn init(window_manager: Rc<RefCell<window_manager::WindowManager>>) -> Self {
        let displays: Vec<Rc<RefCell<dyn Display>>> = Vec::new();
        let curr_pres = Presentation::blank_presentation();

        Self {
            displays,
            curr_pres,
            curr_slide: 0,
            window_manager
        }
    }

    pub fn new_window(&mut self, window_name: String, role: DisplayRole) {
        let window = Window {
            window_name,
            window_manager: self.window_manager.clone(),
            role,
            curr_slide: Slide::blank_slide(),
        };
        self.add_display(window);
    }

    pub fn add_display(&mut self, mut display: impl Display + 'static) {
        display.render(
            self.curr_pres.slides
                .get(self.curr_slide)
                .unwrap_or(&Slide::blank_slide())
        );
        let wrapped_display = Rc::new(RefCell::new(display));
        self.displays.push(wrapped_display);
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
            display.clone().borrow_mut().render(new_slide);
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
