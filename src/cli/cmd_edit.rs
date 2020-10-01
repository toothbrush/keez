use crate::aws;
use crate::cli;
use crate::editor;
use crate::flags;

use flags::operation_mode::OperationMode;

pub fn run(args: cli::Keez, prefix: String, operation_mode: OperationMode) {
    let ps = aws::parameter_store::get_parameters_by_path(prefix, args.debug);

    if args.debug {
        eprintln!("Raw output from Parameter Store:");
        eprintln!("{:?}", ps);
    }

    let original_parameters = ps.unwrap();

    eprintln!(
        "Returned {} parameters from store.",
        original_parameters.parameters().len()
    );

    let after_edit =
        editor::edit_loop::interactive_edit_parameters(original_parameters.clone(), args.debug)
            .unwrap();

    eprintln!("Edited blob contains the following keys:");
    for (key, _param) in after_edit.parameters() {
        eprintln!("  - {}", key);
    }

    aws::parameter_store::push_updated_parameters(original_parameters, after_edit, operation_mode)
        .unwrap();
}
