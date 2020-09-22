use structopt::StructOpt;

mod cli;

fn main() {
    let args = cli::Keez::from_args();
    if args.debug {
        println!("{:?}", args);
    }

    match args.cmd {
        cli::KeezCommand::Export { .. } => {
            cli::cmd_export::cmd_export::run(args);
        }
        _ => {
            println!("Command not yet implemented.");
        }
    }
}
