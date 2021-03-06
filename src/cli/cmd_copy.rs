use crate::aws;
use crate::cli;
use crate::editor;
use crate::flags;

use flags::operation_mode::OperationMode;

// The `copy` command takes a source and target prefix (e.g., /foo and
// /bar) and copies all values under /foo to values at the same
// hierarchy under /bar.  It replaces the initial `/foo' component
// with /bar.  This also works if the target is deep, such as
// /bar/baz/quux.
pub fn run(
    args: cli::Keez,
    source: String,
    destination: String,
    edit: bool,
    operation_mode: OperationMode,
) {
    let parameters =
        aws::parameter_store::get_parameters_by_path(source.clone(), args.debug).unwrap();

    let mut rerooted_parameters =
        aws::parameter_store::reroot_parameters(parameters.clone(), destination.clone()).unwrap();

    if edit {
        rerooted_parameters =
            editor::edit_loop::interactive_edit_parameters(rerooted_parameters.clone(), args.debug)
                .unwrap();
    }

    aws::parameter_store::push_new_parameters(rerooted_parameters, operation_mode).unwrap();
}
