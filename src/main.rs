use clap::{App, Arg};
use smush_material_db::create_database;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let matches = App::new("smush_material_db")
        .version("0.1")
        .author("SMG")
        .about("Create an SQLite material database from SSBH and XMB files.")
        .arg(
            Arg::with_name("input")
                .index(1)
                .short("i")
                .long("input")
                .help("The source folder to search recursively for files")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .short("o")
                .long("output")
                .help("The output SQLite database")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let source_folder = Path::new(matches.value_of("input").unwrap());
    let database_path = Path::new(matches.value_of("output").unwrap());

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
