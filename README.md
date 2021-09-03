# Smush-Material-DB
A Rust CLI program for generating an SQLite database for rendering research.

The program will attempt to parse all `.numatb`, `.numdlb`, `.numshb`, and `.xmb` files in the specified directory. Data will be stored to an SQLite database. The data can be viewed using [SQLite Browser](https://sqlitebrowser.org/). In addition, SQLite bindings exist for many programming languages (C, C#, Rust, Python, etc).

A pregenerated database, python scripts for viewing/export the data from the database, and various value dumps can be found in [Smush-Material-Research](https://github.com/ScanMountGoat/Smush-Material-Research).

## Usage 
`smush_material_db.exe <source folder> <SQLite database output>`  
`smush_material_db.exe "dump directory/root" smush_materials.db`  
`smush_material_db.exe "dump directory/root/items" items.db`  

## Building
`cargo build --release`  

