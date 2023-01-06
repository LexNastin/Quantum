use quantum::{manager::Manager, output::Output};

fn main() {
    let mut manager = Manager::init();
    // println!("Database:\n{}", manager.database.as_string());
    manager.output.new_display(Output::new_window(), quantum::output::DisplayRole::Main);
    manager.output.load_pres(
        manager.database.get_pres(0)
    );
    println!();
    manager.output.next();
    manager.output.next();
    manager.output.next();
    println!();
    manager.output.prev();
    manager.output.prev();
    manager.output.prev();
}
