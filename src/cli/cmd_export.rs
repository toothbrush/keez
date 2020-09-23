use crate::aws;
use crate::cli;

pub fn run(
    _args: cli::Keez,
    _export_filename: std::path::PathBuf,
    _insecure_output: bool,
    source: String,
) {
    let ps = aws::parameter_store::get_parameters_by_path(source);

    let debug = true; //TODO proper settings
    if debug {
        println!("Raw output from Parameter Store:");
        println!("{:?}", ps);
    }

    println!(
        "Returned {:?} parameters from store.",
        ps.unwrap().get_params().len()
    );
}
