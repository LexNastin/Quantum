use nannou::prelude::*;
use std::collections::HashMap;
use crate::presentation::Slide;

struct WindowModel {
    animation_state: AnimationState,
    window: window::Id,
}

enum Freeze {
    Hold { hold_time: i32 },
    Continue,
}

struct AnimationProgress {
    start_time: i32,
    duration: i32,
}

enum Animation {
    Fade,
    Merge,
}

enum AnimationState {
    Animating { from: Slide, to: Slide, animation: Animation, progress: AnimationProgress },
    Stop(Slide),
}

pub enum WindowType {
    Fullscreen { display_number: i32 },
    Windowed { position: (i32, i32), size: (i32, i32) },
}

pub struct Window {
    pub title: String,
    pub window_type: WindowType,
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
            window_type: WindowType::Windowed { position, size },
        }
    }

    pub fn new_fullscreen(title: String, display_number: i32) -> Self {
        let id = nannou::app(Self::slide_model).update(Self::update).run();
        Self {
            title,
            window_type: WindowType::Fullscreen { display_number },
        }
    }

    fn slide_model(app: &App) -> WindowModel {
        let window = app.new_window().view(Self::view_slide).build().unwrap();
        WindowModel {
            animation_state: AnimationState::Stop(Slide::blank_slide()),
            window,
        }
    }

    fn update(_app: &App, _model: &mut WindowModel, _update: Update) {

    }

    fn view_slide(app: &App, _model: &WindowModel, frame: Frame) {
        let draw = app.draw();
        draw.background().color(PLUM);
        draw.ellipse().color(STEELBLUE);
        draw.to_frame(app, &frame).unwrap();
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
