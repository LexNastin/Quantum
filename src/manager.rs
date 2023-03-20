use crate::database::Database;
use crate::output::Output;
use crate::window_manager::{WindowManager, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Manager {
    pub database: Database,
    pub output: Output,
    pub window_manager: Rc<RefCell<WindowManager>>,
}

impl Manager {
    pub fn init() -> Self {
        let database = Database::load_db();
        let window_manager = Rc::new(RefCell::new(WindowManager::init()));
        let output = Output::init(window_manager.clone());
        // TODO: create main menu window
//         window_manager
//             .borrow_mut()
//             .add_window(
//                 "main".to_owned(),
//                 Window::new_windowed(
//                     "QS - Main Menu".to_owned(),
//                     (640, 480)
//                 )
//             );

        Self {
            database,
            output,
            window_manager,
        }
    }
}
