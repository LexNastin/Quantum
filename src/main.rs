use quantum::{manager::Manager, output::DisplayRole};

fn main() {
    let mut manager = Manager::init();

    // println!("Database:\n{}", manager.database.as_string());
    manager.output.new_window("output 1".to_owned(), DisplayRole::Main);
    manager.output.load_pres(
        manager.database.get_pres(0)
    );
//    println!();
//    manager.output.next();
//    manager.output.next();
//    manager.output.next();
//    println!();
//    manager.output.prev();
//    manager.output.prev();
//    manager.output.prev();
}
