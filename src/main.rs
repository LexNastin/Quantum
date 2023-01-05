use quantum::database::Database;

fn main() {
    let db: Database = Database::load_db();
    println!("Database:");
    println!("{}", db.as_string());
}
