use crate::aws;
use crate::cli;
use crate::secrets;

pub fn run(
    args: cli::Keez,
    _export_filename: std::path::PathBuf,
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
        "Returned {:?} parameters from store.",
        unwrapped_parameterblob.get_params().len()
    );

    let yaml_blob = serde_yaml::to_string(&unwrapped_parameterblob).unwrap();

    if insecure_output {
        println!("{}", yaml_blob);
    }

    let key = secrets::keychain_access::get_symmetric_key();

    if args.debug {
        println!("Found symmetric key = {:?}", key);
    }

    let encrypted_form = secrets::symmetric_store::encrypt(yaml_blob).unwrap();

    if args.debug {
        println!("encrypted_form = {:?}", encrypted_form);
    }
}
