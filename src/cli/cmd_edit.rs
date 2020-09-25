use crate::aws;
use crate::cli;
use crate::editor;

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

    let new_yaml_blob = editor::edit_loop::interactive_edit(yaml_blob).unwrap();

    if args.debug {
        println!("New YAML blob:");
        println!("{}", new_yaml_blob);
    }
    // TODO re-open editor if something about the new YAML makes it
    // unparsable, or if something goes wrong pushing to AWS API.

    // Deserialize it back to a Rust type.
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&new_yaml_blob).unwrap();

    if args.debug {
        println!("Data structure after deserialization:");
        println!("{:?}", deserialized);
    }

    println!("Edited blob contains the following keys:");
    for (key, _param) in deserialized.get_parameters() {
        println!("  - {}", key);
    }

    let write_mode = !args.dry_run; // TODO proper enum OperationMode with READ_ONLY vs READ_WRITE

    aws::parameter_store::push_updated_parameters(
        unwrapped_parameterblob,
        deserialized,
        write_mode,
    )
    .unwrap();
}
