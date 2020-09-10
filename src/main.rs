use smush_material_db::create_database;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: smush_material_db.exe <source folder> <output SQLite database>");
        return;
    }
    let source_folder = Path::new(&args[1]);
    let database_path = Path::new(&args[2]);

    // Overwrite the file if it already exists.
    if database_path.exists() {
        fs::remove_file(database_path).unwrap();
    }

    let duration = Instant::now();
    match create_database(source_folder, database_path) {
        Ok(_) => {}
        Err(e) => println!("Error encountered while creating database: {:?}", e),
    }
    println!("Total: {:?}", duration.elapsed());
}
