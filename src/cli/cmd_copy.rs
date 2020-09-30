use crate::aws;
use crate::cli;

// The `copy` command takes a source and target prefix (e.g., /foo and
// /bar) and copies all values under /foo to values at the same
// hierarchy under /bar.  It replaces the initial `/foo' component
// with /bar.  This also works if the target is deep, such as
// /bar/baz/quux.
pub fn run(args: cli::Keez, source: String, destination: String) {
    let write_mode = !args.dry_run; // TODO proper enum OperationMode with READ_ONLY vs READ_WRITE
    let parameters = aws::parameter_store::get_parameters_by_path(source.clone()).unwrap();

    let rerooted_parameters =
        aws::parameter_store::reroot_parameters(parameters.clone(), destination.clone()).unwrap();
    aws::parameter_store::push_new_parameters(rerooted_parameters, write_mode).unwrap();
}
