use std::env;
use std::fs;
use std::path::Path;

use crate::aws;
use crate::cli;
use crate::flags;
use crate::secrets;

use flags::operation_mode::OperationMode;

pub fn run(
    args: cli::Keez,
    export_filename: std::path::PathBuf,
    insecure_output: bool,
    source: String,
    operation_mode: OperationMode,
) {
    let ps = aws::parameter_store::get_parameters_by_path(source, args.debug);

    if args.debug {
        eprintln!("Raw output from Parameter Store:");
        eprintln!("{:?}", ps);
    }

    let unwrapped_parameterblob = ps.unwrap();

    eprintln!(
        "Returned {} parameters from store.",
        unwrapped_parameterblob.get_parameters().len()
    );

    let yaml_blob = serde_yaml::to_string(&unwrapped_parameterblob).unwrap();

    if insecure_output {
        eprintln!("{}", yaml_blob);
    }

    if args.debug {
        let key = secrets::keychain_access::get_symmetric_key();
        eprintln!("Found symmetric key = {:?}", key);
    }

    let encrypted_form = secrets::symmetric_store::encrypt(yaml_blob).unwrap();

    // Create a path to the desired file
    let path = Path::new(&export_filename);
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    };

    if operation_mode == OperationMode::ReadWrite {
        eprint!(
            "Writing exported parameters to {}... ",
            absolute_path.display()
        );

        // Open file and create if necessary.  We'll overwrite any
        // existing file at the given path.
        fs::write(&absolute_path, &encrypted_form).unwrap();

        eprintln!("done.");
    } else if operation_mode == OperationMode::ReadOnly {
        eprintln!(
            "[DRY-RUN] Would write exported parameters to {}... ",
            absolute_path.display()
        );
    }
}
