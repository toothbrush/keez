use structopt::StructOpt;

mod aws;
mod cli;
mod editor;
mod flags;
mod secrets;

fn main() {
    let args = cli::Keez::from_args();
    if *args.debug() {
        eprintln!("{:?}", args);
    }

    match &*args.cmd() {
        cli::KeezCommand::Export {
            export_filename,
            insecure_output,
            source,
        } => {
            cli::cmd_export::run(
                args.clone(),
                export_filename.clone(),
                insecure_output.clone(),
                source.clone(),
                args.operation_mode().clone(),
            );
        }
        cli::KeezCommand::Import {
            import_filename,
            destination,
            edit,
        } => {
            cli::cmd_import::run(
                args.clone(),
                import_filename.clone(),
                destination.clone(),
                edit.clone(),
                args.operation_mode().clone(),
            );
        }
        cli::KeezCommand::Edit { prefix } => {
            cli::cmd_edit::run(args.clone(), prefix.clone(), args.operation_mode().clone());
        }
        cli::KeezCommand::Copy {
            source,
            destination,
            edit,
        } => {
            cli::cmd_copy::run(
                args.clone(),
                source.clone(),
                destination.clone(),
                edit.clone(),
                args.operation_mode().clone(),
            );
        }
        cli::KeezCommand::Create {} => {
            cli::cmd_create::run(args.clone(), args.operation_mode().clone());
        }
    }
}
