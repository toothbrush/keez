use std::fs;

use mktemp::Temp;

use crate::aws;
use crate::cli;

pub fn run(args: cli::Keez, prefix: String) {
    let ps = aws::parameter_store::get_parameters_by_path(prefix);

    if args.debug {
        println!("Raw output from Parameter Store:");
        println!("{:?}", ps);
    }

    let unwrapped_parameterblob = ps.unwrap();

    println!(
        "Returned {} parameters from store.",
        unwrapped_parameterblob.get_parameters().len()
    );

    let yaml_blob = serde_yaml::to_string(&unwrapped_parameterblob).unwrap();

    // Create a temporary file somewhere.  When the variable goes out
    // of scope, the mktemp crate takes care of cleaning it up.
    let temp_file = Temp::new_file().unwrap();
    // Write YAML blob to temp file, then edit.
    fs::write(&temp_file, &yaml_blob).unwrap();

    println!("Opening {} for editing...", temp_file.display());
}
