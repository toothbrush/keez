use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(Debug, StructOpt)]
#[structopt(about = "simple manipulation of AWS SSM Parameter Store values")]
struct Keez {
    // Define our global flags here.
    #[structopt(short = "n", long)]
    dry_run: bool,
    #[structopt(subcommand)]
    cmd: KeezCommand,
}

#[derive(Debug, StructOpt)]
enum KeezCommand {
    Copy {
        source: String,
        destination: String,
    },
    Create {},
    Edit {
        prefix: String,
    },
    Export {
        source: String,
        #[structopt(long, parse(from_os_str))]
        export_filename: PathBuf,
        #[structopt(short = "I", long)]
        insecure_output: bool,
    },
    Import {
        #[structopt(long, parse(from_os_str))]
        import_filename: PathBuf,
        #[structopt(short, long)]
        edit: bool,
    },
}

fn main() {
    let args = Keez::from_args();
    println!("{:?}", args);

    let mut rt = Runtime::new().expect("failed to initialize runtime");
    let conf = envy_store::from_path::<HashMap<String, String>, _>("/demo");
    println!("config {:#?}", rt.block_on(conf))
}
