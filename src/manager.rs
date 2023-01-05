use crate::database::Database;
use crate::output::Output;

pub struct Manager {
    pub database: Database,
    pub output: Output,
}

impl Manager {
    pub fn init() -> Manager {
        let database = Database::load_db();
        let output = Output::init();

        Manager {
            database,
            output,
        }
    }
}
