use quantum::manager::Manager;

fn main() {
    let manager = Manager::init();
    println!("Database:");
    println!("{}", manager.database.as_string());
}
