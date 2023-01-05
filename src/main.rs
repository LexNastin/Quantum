use quantum::database::Database;

fn main() {
    let db: Database = Database::load_db();
    println!("{}", db.presentations[0].slides[0].text);
}
