use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use crate::aws;
use crate::cli;
use crate::secrets;

pub fn run(
    args: cli::Keez,
    export_filename: std::path::PathBuf,
    insecure_output: bool,
    source: String,
) {
    let ps = aws::parameter_store::get_parameters_by_path(source);

    if args.debug {
        println!("Raw output from Parameter Store:");
        println!("{:?}", ps);
    }

    let unwrapped_parameterblob = ps.unwrap();

    println!(
        "Returned {} parameters from store.",
        unwrapped_parameterblob.get_params().len()
    );

    let yaml_blob = serde_yaml::to_string(&unwrapped_parameterblob).unwrap();

    if insecure_output {
        println!("{}", yaml_blob);
    }

    if args.debug {
        let key = secrets::keychain_access::get_symmetric_key();
        println!("Found symmetric key = {:?}", key);
    }

    let encrypted_form = secrets::symmetric_store::encrypt(yaml_blob).unwrap();

    // Create a path to the desired file
    let path = Path::new(&export_filename);
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().unwrap().join(path)
    };

    print!(
        "Writing exported parameters to {}... ",
        absolute_path.display()
    );

    // Open file and create if necessary.  We'll overwrite any
    // existing file at the given path.
    fs::write(&absolute_path, &encrypted_form).unwrap();

    println!("done.");
}
