use crate::aws;
use crate::cli;
use crate::editor;
use crate::flags;

use flags::operation_mode::OperationMode;

pub fn run(args: cli::Keez, operation_mode: OperationMode) {
    // Create an example blob of YAML for the user to ape:
    let example = String::from(
        "---
parameters:
  /this/is/one:
    value: foo
    type: String
  /this/is/another:
    value: bar
    type: SecureString
  /different:
    value: baz
    type: SecureString
",
    );

    let new_yaml_blob = editor::edit_loop::interactive_edit(example).unwrap();

    if args.debug {
        eprintln!("New YAML blob:");
        eprintln!("{}", new_yaml_blob);
    }
    // TODO re-open editor if something about the new YAML makes it
    // unparsable, or if something goes wrong pushing to AWS API.

    // Deserialize it back to a Rust type.
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&new_yaml_blob).unwrap();

    if args.debug {
        eprintln!("Data structure after deserialization:");
        eprintln!("{:?}", deserialized);
    }

    eprintln!("Create blob contains the following keys:");
    for (key, _param) in deserialized.get_parameters() {
        eprintln!("  - {}", key);
    }

    aws::parameter_store::push_new_parameters(deserialized, operation_mode).unwrap();
}
