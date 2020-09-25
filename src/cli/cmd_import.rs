use std::env;
use std::fs;
use std::path::Path;

use crate::aws;
use crate::cli;
use crate::editor;
use crate::secrets;

pub fn run(args: cli::Keez, import_filename: std::path::PathBuf, destination: String, edit: bool) {
    // Create a path to the desired file
    let path = Path::new(&import_filename);
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    };

    eprintln!(
        "Reading exported parameters from {}... ",
        absolute_path.display()
    );

    // Open file read-only.
    // Read the file contents for decrypting.
    let encrypted_blob: Vec<u8> = fs::read(absolute_path).unwrap();

    let raw_yaml = secrets::symmetric_store::decrypt(encrypted_blob).unwrap();

    if args.debug {
        eprintln!("Read YAML from encrypted file:");
        eprintln!("{}", raw_yaml);
    }

    if edit {
        // Let the user edit the ...
        // it's not that simple.  we need to rewrite the paths, first.
        // raw_yaml = editor::edit_loop::interactive_edit(raw_yaml).unwrap();

        panic!("oh no, the edit-before-import command isn't implemented yet!")
    }

    // Deserialize it back to a Rust type.
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&raw_yaml).unwrap();

    if args.debug {
        eprintln!("Data structure after deserialization:");
        eprintln!("{:?}", deserialized);
    }

    eprintln!("Imported blob contains the following keys:");
    for (key, _param) in deserialized.get_parameters() {
        eprintln!("  - {}", key);
    }
    eprintln!("\nWe'll rewrite the path prefix:");
    eprintln!("  {} => {}\n", deserialized.get_path_prefix(), destination);

    let write_mode = !args.dry_run; // TODO proper enum OperationMode with READ_ONLY vs READ_WRITE
    aws::parameter_store::migrate_parameters(deserialized, destination, write_mode).unwrap();
}
