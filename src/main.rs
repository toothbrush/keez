use structopt::StructOpt;

mod aws;
mod cli;

fn main() {
    let args = cli::Keez::from_args();
    if *args.get_debug() {
        println!("{:?}", args);
    }

    match *args.get_cmd() {
        cli::KeezCommand::Export { .. } => {
            cli::cmd_export::run(args);
        }
        _ => {
            println!("Command not yet implemented.");
        }
    }
}
