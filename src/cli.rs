pub mod cmd_create;
pub mod cmd_edit;
pub mod cmd_export;
pub mod cmd_import;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(about = "simple manipulation of AWS SSM Parameter Store values")]
pub struct Keez {
    // Define our global flags here.
    #[structopt(short = "n", long)]
    /// Avoid any write operations on the Parameter Store.
    dry_run: bool,
    #[structopt(short = "d", long)]
    /// Provide extra detailed output.
    debug: bool,
    #[structopt(subcommand)]
    cmd: KeezCommand,
}

impl Keez {
    // useful for setting defaults? TODO?
    // pub fn new() -> Foo { Foo { x: 0 } }

    // We create the getter function here:
    pub fn get_debug(&self) -> &bool {
        &self.debug
    }

    pub fn get_cmd(&self) -> &KeezCommand {
        &self.cmd
    }
}

// TODO add long-form example usage text to subcommands.
#[derive(Clone, Debug, StructOpt)]
pub enum KeezCommand {
    /// Transplant all parameters under a given prefix to another prefix.
    Copy {
        /// The path prefix for selecting parameters to copy.
        source: String,
        /// The path where you would like to copy parameters to.
        destination: String,
    },
    /// Interactively create a new set of parameters.
    Create {},
    /// Interactively edit existing parameters under a given prefix.
    Edit {
        /// The path prefix for selecting parameters to edit.
        prefix: String,
    },
    /// Export is useful for migrating a group of parameters to another AWS account or region.
    Export {
        /// The path prefix for selecting parameters to export.
        source: String,
        #[structopt(long, parse(from_os_str))]
        /// File to export parameters to, prior to importing to another account.
        export_filename: PathBuf,
        #[structopt(short = "I", long)]
        /// For debugging: print results of export to stdout.
        insecure_output: bool,
    },
    /// Import is to be used with the output from the keez-export command.
    Import {
        /// The target path for importing parameters.
        destination: String,
        #[structopt(long, parse(from_os_str))]
        /// File to import parameters from, obtained by exporting with keez-export.
        import_filename: PathBuf,
        #[structopt(short, long)]
        /// Whether to interactively edit values prior to importing.
        edit: bool, // TODO implement interactive edit pre-upload.
    },
}
