pub mod cmd_copy;
pub mod cmd_create;
pub mod cmd_edit;
pub mod cmd_export;
pub mod cmd_import;

use std::path::PathBuf;
use structopt::StructOpt;

use crate::flags::operation_mode::OperationMode;

#[derive(Clone, Debug, StructOpt)]
/// simple & interactive manipulation of AWS SSM Parameter Store values
pub struct Keez {
    // Define our global flags here.
    #[structopt(short = "n", long)]
    /// Avoid any write operations on the Parameter Store.
    ///
    /// This is helpful when you want to know which parameters will be
    /// affected by a given operation.  Read-only access to AWS SSM is
    /// still performed.
    dry_run: bool,
    #[structopt(short = "d", long)]
    /// Provide extra detailed output.
    ///
    /// This might help you pinpoint problems, and will dump various
    /// intermediate data structures along the way.  Be careful,
    /// sensitive information might end up in your terminal.
    debug: bool,
    #[structopt(subcommand)]
    cmd: KeezCommand,
}

impl Keez {
    pub fn debug(&self) -> &bool {
        &self.debug
    }

    pub fn cmd(&self) -> &KeezCommand {
        &self.cmd
    }

    pub fn operation_mode(&self) -> &OperationMode {
        if self.dry_run {
            &OperationMode::ReadOnly
        } else {
            &OperationMode::ReadWrite
        }
    }
}

#[derive(Clone, Debug, StructOpt)]
pub enum KeezCommand {
    /// Transplant all parameters under a given prefix to another prefix
    ///
    /// This command recursively queries all parameters with path
    /// prefix <source> from the AWS Systems Manager Parameter Store.
    /// It then tries to place each of them into an equivalent spot in
    /// a new tree with prefix <destination>.
    ///
    /// For example:{n}
    /// keez copy /preprod /prod-eu/baz
    ///
    /// For example, if you run the above command, and these
    /// parameters exist:{n}
    /// - /preprod/foo{n}
    /// - /preprod/bar
    ///
    /// They will be copied to new locations:{n}
    /// - /prod-eu/baz/foo{n}
    /// - /prod-eu/baz/bar
    ///
    /// The prefix <source> is replaced by the prefix <destination>.
    /// Note that the operation will fail if the target parameter
    /// store values already exist, that is, the operations are run
    /// with overwriting set to "disabled".
    Copy {
        /// The path prefix for selecting parameters to copy.
        source: String,
        /// The path where you would like to copy parameters to.
        destination: String,
        #[structopt(short, long)]
        /// Whether to interactively edit values prior to importing.
        edit: bool,
    },
    /// Interactively create parameters in bulk
    ///
    /// This command aims to make it easy to create a slew of
    /// parameters in AWS Systems Manager Parameter Store in one go.
    /// It will spawn an editor session with some example YAML, which
    /// you can adapt for your needs.  Once you save and exit the
    /// editor, the parameters will be created as specified.
    ///
    /// If you'd like to modify them after the fact, see the `edit`
    /// subcommand.
    ///
    /// This command respects your $EDITOR environment variable.  If
    /// you don't want to modify anything, simply close your editor
    /// without changing the file and the process will be aborted.
    Create {},
    /// Interactively edit existing parameters under a given prefix
    ///
    /// This command recursively queries all parameters from the AWS
    /// Systems Manager Parameter Store by path.  It then spawns an
    /// editor session in which you can modify the parameters.  If you
    /// write & save, keez pushes those edits back to Parameter Store.
    /// This allows for easy modification of the parameter values.
    /// Beats using the AWS Console amirite?!
    ///
    /// For example:{n}
    /// keez edit /path/prefix/foo
    ///
    /// This command respects your $EDITOR environment variable.  If you don't
    /// want to modify anything, simply close your editor without changing the
    /// file and the process will be aborted.
    Edit {
        /// The path prefix for selecting parameters to edit.
        prefix: String,
    },
    /// Export is useful for migrating a group of parameters to another AWS account or region.
    ///
    /// This command recursively queries all parameters with prefix
    /// <source> from the AWS Systems Manager Parameter Store.  This
    /// allows for easy migration of the parameter values to another
    /// AWS account or region, or simply to another app environment,
    /// when a straight `keez copy` won't work.  The exported
    /// parameters are written to a file, encrypted.  They're
    /// symmetrically encrypted using a key automatically generated
    /// and stored in your system keychain.  This is to avoid the risk
    /// of random secrets remaining lying around in your home
    /// directory.
    ///
    /// For example:{n}
    /// keez export --export-filename ./foo.yaml.enc /path/prefix/foo
    ///
    /// After exporting, you can use the `import` command to push the
    /// parameters somewhere new, possibly to another AWS account or
    /// region.
    ///
    /// Note that if you want to copy parameters within one AWS region
    /// and account, you can simply use `keez copy`.
    Export {
        /// The path prefix for selecting parameters to export.
        source: String,
        #[structopt(long, parse(from_os_str))]
        /// File to export parameters to, prior to importing to another account.
        export_filename: PathBuf,
        #[structopt(short = "I", long)]
        /// For debugging: print results of export to stdout.
        ///
        /// Use this option with care, because you risk exposing
        /// secrets.
        insecure_output: bool,
    },
    /// Import parameters from a previous `keez export`.
    ///
    /// This command is useful for migrating parameters cross-account.
    /// It works with an export file generated by the `keez export`
    /// command.  This file is expected to be encrypted with a key
    /// stored in your system keychain - but don't worry, this should
    /// be handled transparently for you by keez.
    ///
    /// Once you have a dump of a set of parameters from another
    /// account, import them as follows.  Remember to specify
    /// <destination>, as a replacement for the prefix path that you
    /// exported from.
    ///
    /// Usage:{n}
    /// keez import --import-filename ./foo.yaml.enc /path/prefix/foo
    ///
    /// For example, if you exported from another account, with a query of
    /// /foo, and that resulted in the following exported parameters:{n}{n}
    /// prefix: /foo{n}
    /// parameters:{n}
    /// - /foo/a{n}
    /// - /foo/b
    ///
    /// Now, you need to specify a prefix for importing, too.  If you
    /// specify the exact same one as when exporting, you'll end up
    /// with the same set of parameters as in the old account.
    /// However, you can also decide to put them elsewhere.  Let's say
    /// i specify a new prefix of /baz/quux; this will result in the
    /// following parameters in the new account:{n}{n}
    /// - /baz/quux/a{n}
    /// - /baz/quux/b
    ///
    /// Notice how the original prefix, /foo, has been replaced by the
    /// new prefix you specified, /baz/quux.
    Import {
        /// The target path for importing parameters.
        destination: String,
        #[structopt(long, parse(from_os_str))]
        /// File to import parameters from, obtained by exporting with `keez export`.
        import_filename: PathBuf,
        #[structopt(short, long)]
        /// Whether to interactively edit values prior to importing.
        edit: bool,
    },
}
