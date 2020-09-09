use rusqlite::Connection;
use smush_material_db::*;
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

    // TODO: Move database/SQL code to lib.rs.
    let mut connection = Connection::open(database_path).unwrap();

    initialize_database(&mut connection).unwrap();
    process_files(&source_folder, &mut connection).unwrap();
    create_indexes(&mut connection).unwrap();

    println!("Total: {:?}", duration.elapsed());
}
