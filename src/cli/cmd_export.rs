use crate::aws;
use crate::cli;

pub fn run(
    _args: cli::Keez,
    _export_filename: std::path::PathBuf,
    _insecure_output: bool,
    source: String,
) {
    let ps = aws::parameter_store::get_parameters_by_path(source);
    println!("{:?}", ps);
}
