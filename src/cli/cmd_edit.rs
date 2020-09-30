use crate::aws;
use crate::cli;
use crate::editor;

pub fn run(args: cli::Keez, prefix: String) {
    let ps = aws::parameter_store::get_parameters_by_path(prefix, args.debug);

    if args.debug {
        eprintln!("Raw output from Parameter Store:");
        eprintln!("{:?}", ps);
    }

    let original_parameters = ps.unwrap();

    eprintln!(
        "Returned {} parameters from store.",
        original_parameters.get_parameters().len()
    );

    let after_edit =
        editor::edit_loop::interactive_edit_parameters(original_parameters.clone(), args.debug)
            .unwrap();

    eprintln!("Edited blob contains the following keys:");
    for (key, _param) in after_edit.get_parameters() {
        eprintln!("  - {}", key);
    }

    let write_mode = !args.dry_run; // TODO proper enum OperationMode with READ_ONLY vs READ_WRITE

    aws::parameter_store::push_updated_parameters(original_parameters, after_edit, write_mode)
        .unwrap();
}
