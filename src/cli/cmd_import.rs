use std::env;
use std::fs;
use std::path::Path;

use crate::aws;
use crate::cli;
use crate::secrets;

pub fn run(args: cli::Keez, import_filename: std::path::PathBuf, destination: String, _edit: bool) {
    // Create a path to the desired file
    let path = Path::new(&import_filename);
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    };

    println!(
        "Reading exported parameters from {}... ",
        absolute_path.display()
    );

    // Open file read-only.
    // Read the file contents for decrypting.
    let encrypted_blob: Vec<u8> = fs::read(absolute_path).unwrap();
    println!("{:?}", encrypted_blob);

    let raw_yaml = secrets::symmetric_store::decrypt(encrypted_blob).unwrap();
    println!("{}", raw_yaml);
}
