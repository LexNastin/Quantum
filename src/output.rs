use crate::presentation::Presentation;
use crate::presentation::Slide;
use crate::window_manager;
use std::string::String;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub enum DisplayRole {
    Main,
    Foldback,
    Other(String),
}

pub enum ActionResult {
    Success,
    DisplayMissing,
    Failure,
    NoMoreSlides,
    OnFirstSlide,
}

impl ActionResult {
    fn from_bool(result: bool) -> Self {
        if result { Self::Success } else { Self::Failure }
    }
}

pub trait Display {
    fn render(&mut self, new_slide: &Slide) -> ActionResult;
    fn destroy(&mut self) -> ActionResult;
}

pub struct Window {
    pub window_name: String,
    window_manager: Rc<RefCell<window_manager::WindowManager>>,
    pub role: DisplayRole,
    pub curr_slide: Slide,
}

impl Display for Window {
    fn render(&mut self, new_slide: &Slide) -> ActionResult {
        println!("Display output: {}", self.curr_slide.text);
        if let Some(window) = self.window_manager.borrow().get_window(&self.window_name) {
            let result = window.render_slide(&self.curr_slide, new_slide);
            self.curr_slide = new_slide.clone();
            ActionResult::from_bool(result)
        } else {
            ActionResult::DisplayMissing
        }
    }

    fn destroy(&mut self) -> ActionResult {
        if let Some(window) = self.window_manager.borrow().get_window(&self.window_name) {
            let result = window.close(self.window_name.clone());
            ActionResult::from_bool(result)
        } else {
            ActionResult::DisplayMissing
        }
    }
}

pub struct Output {
    pub displays: HashMap<String, Rc<RefCell<dyn Display>>>,
    pub curr_pres: Presentation,
    pub curr_slide: usize,
    window_manager: Rc<RefCell<window_manager::WindowManager>>,
}

impl Output {
    pub fn init(window_manager: Rc<RefCell<window_manager::WindowManager>>) -> Self {
        let displays: HashMap<String, Rc<RefCell<dyn Display>>> = HashMap::new();
        let curr_pres = Presentation::blank_presentation();

        Self {
            displays,
            curr_pres,
            curr_slide: 0,
            window_manager
        }
    }

    pub fn new_window(&mut self, window_name: String, role: DisplayRole) {
        // TODO: don't use hardcoded display value
        let wm_window = window_manager::Window::new_fullscreen("QS - Output".to_owned(), 0);
        self.window_manager.borrow_mut().add_window(window_name.clone(), wm_window);
        let window = Window {
            window_name: window_name.clone(),
            window_manager: self.window_manager.clone(),
            role,
            curr_slide: Slide::blank_slide(),
        };
        self.add_display(window_name, window);
    }

    pub fn add_display(&mut self, name: String, mut display: impl Display + 'static) {
        display.render(
            self.curr_pres.slides
                .get(self.curr_slide)
                .unwrap_or(&Slide::blank_slide())
        );
        let wrapped_display = Rc::new(RefCell::new(display));
        self.displays.insert(name, wrapped_display);
    }

    pub fn remove_display(&mut self, name: String) -> ActionResult {
        let result = if let Some(display) = self.displays.get(&name) {
            display.borrow_mut().destroy()
        } else {
            ActionResult::DisplayMissing
        };
        self.displays.remove(&name);
        result
    }

    pub fn load_pres(&mut self, new_pres: Presentation) -> ActionResult {
        println!("Loaded presentation: {}", new_pres.title);
        let result = self.render(
            new_pres.slides
                .get(self.curr_slide)
                .unwrap_or(&Slide::blank_slide())
        );
        self.curr_pres = new_pres;
        result
    }

    fn render(&mut self, new_slide: &Slide) -> ActionResult {
        let mut result = ActionResult::Success;
        let mut to_remove: Vec<String> = Vec::new();
        for (name, display) in &self.displays {
            let mut borrowed_display = display.borrow_mut();
            let render_result = borrowed_display.render(new_slide);
            match render_result {
                ActionResult::Failure => result = render_result,
                ActionResult::DisplayMissing => to_remove.push(name.clone()),
                _ => { }
            };
        };
        for name in to_remove {
            self.remove_display(name);
        }
        result
    }

    pub fn next(&mut self) -> ActionResult {
        println!("Attempting next slide...");
        let length = self.curr_pres.slides.len();
        if self.curr_slide + 1 < length {
            self.curr_slide += 1;
            self.render(
                self.curr_pres.slides.clone()
                    .get(self.curr_slide)
                    .unwrap_or(&Slide::blank_slide())
            )
        } else {
            ActionResult::NoMoreSlides
        }
    }

    pub fn prev(&mut self) -> ActionResult {
        println!("Attempting prev slide...");
        if self.curr_slide > 0 {
            self.curr_slide -= 1;
            self.render(
                self.curr_pres.slides.clone()
                    .get(self.curr_slide)
                    .unwrap_or(&Slide::blank_slide())
            )
        } else {
            ActionResult::OnFirstSlide
        }
    }
}
