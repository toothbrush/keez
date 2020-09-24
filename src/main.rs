use structopt::StructOpt;

mod aws;
mod cli;
mod secrets;

fn main() {
    let args = cli::Keez::from_args();
    if *args.get_debug() {
        println!("{:?}", args);
    }

    match &*args.get_cmd() {
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
            );
        }
        _ => {
            println!("Command not yet implemented.");
        }
    }
}
