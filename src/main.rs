use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(Debug, StructOpt)]
#[structopt(about = "the well-known content tracker")]
enum Git {
    Add {
        #[structopt(short)]
        interactive: bool,
        #[structopt(short)]
        patch: bool,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Fetch {
        #[structopt(long)]
        dry_run: bool,
        #[structopt(long)]
        all: bool,
        repository: Option<String>,
    },
    Commit {
        #[structopt(short)]
        message: Option<String>,
        #[structopt(short)]
        all: bool,
    },
}

#[derive(Deserialize, Debug)]
struct Config {
    foo: String,
    bar: Vec<String>,
    zar: u32,
}

fn main() {
    let matches = Git::from_args();
    println!("{:?}", matches);
    match matches {
        Git::Add { .. } => {}
        Git::Fetch { .. } => {}
        Git::Commit { .. } => {}
    }

    let mut rt = Runtime::new().expect("failed to initialize runtime");
    let conf = envy_store::from_path::<Config, _>("/demo");
    println!("config {:#?}", rt.block_on(conf))
}
