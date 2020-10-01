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
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&example).unwrap();

    let new_parameter_blob =
        editor::edit_loop::interactive_edit_parameters(deserialized, args.debug).unwrap();

    if args.debug {
        eprintln!("New parameter blob:");
        eprintln!("{:?}", new_parameter_blob);
    }

    eprintln!("Create blob contains the following keys:");
    for (key, _param) in new_parameter_blob.parameters() {
        eprintln!("  - {}", key);
    }

    aws::parameter_store::push_new_parameters(new_parameter_blob, operation_mode).unwrap();
}
