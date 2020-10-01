use std::env;
use std::fs;
use std::path::Path;

use crate::aws;
use crate::cli;
use crate::editor;
use crate::flags;
use crate::secrets;

use flags::operation_mode::OperationMode;

pub fn run(
    args: cli::Keez,
    import_filename: std::path::PathBuf,
    destination: String,
    edit: bool,
    operation_mode: OperationMode,
) {
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

    // Deserialize it back to a Rust type.
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&raw_yaml).unwrap();

    eprintln!("Imported blob contains the following keys:");
    for (key, _param) in deserialized.get_parameters() {
        eprintln!("  - {}", key);
    }
    eprintln!("\nWe'll rewrite the path prefix:");
    eprintln!("  {} => {}\n", deserialized.get_path_prefix(), destination);

    if args.debug {
        eprintln!("Data structure after deserialization:");
        eprintln!("{:?}", deserialized);
    }

    let mut rerooted = aws::parameter_store::reroot_parameters(deserialized, destination).unwrap();

    if edit {
        rerooted =
            editor::edit_loop::interactive_edit_parameters(rerooted.clone(), args.debug).unwrap();
    }

    aws::parameter_store::push_new_parameters(rerooted, operation_mode).unwrap();
}
