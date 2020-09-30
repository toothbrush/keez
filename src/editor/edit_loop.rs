use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::process::Command;
use std::process::Stdio;

use mktemp::Temp;
use text_io::read;

use crate::aws;

#[derive(Debug)]
enum EditError {
    EditorCommandError,
}
impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EditError::EditorCommandError => write!(f, "editor error!"),
        }
    }
}
impl error::Error for EditError {}

pub fn interactive_edit_parameters(
    params: aws::parameter_store::ParameterCollection,
    debug: bool,
) -> Result<aws::parameter_store::ParameterCollection, Box<dyn error::Error>> {
    let mut yaml_blob = serde_yaml::to_string(&params)?;

    loop {
        let mut new_yaml_blob = interactive_edit(yaml_blob.clone())?;

        if debug {
            eprintln!("New YAML blob after edit session:");
            eprintln!("{}", new_yaml_blob);
        }

        // Deserialize it back to a Rust type.
        match serde_yaml::from_str(&new_yaml_blob) {
            Ok(pc) => {
                if debug {
                    eprintln!("Data structure after deserialization:");
                    eprintln!("{:?}", pc);
                }
                return Ok(pc);
            }
            Err(whatever) => {
                eprintln!("Uh oh, there was a problem parsing the YAML you provided.");
                eprintln!("If you like, we'll provide another opportunity to continue editing, and maybe fix up the mistake.");
                eprintln!("The error was: {}", whatever);
                eprintln!("Press enter to edit again, or C-c to exit...");
                // reads until a \n is encountered
                let _line: String = read!("{}\n");
            }
        };
        // okay, next time around we want to present the user with the
        // thing that didn't parse.
        yaml_blob = new_yaml_blob.clone();
    }
}

pub fn interactive_edit(text: String) -> Result<String, Box<dyn error::Error>> {
    let editor = find_editor();

    // Create a temporary file somewhere.  When the variable goes out
    // of scope, the mktemp crate takes care of cleaning it up.
    let temp_file = Temp::new_file().unwrap();
    let path_buf = temp_file.to_path_buf();
    // Write YAML blob to temp file, then edit.
    fs::write(&temp_file, &text).unwrap();

    eprintln!("Opening {} for editing...", temp_file.display());

    let output = Command::new(editor)
        .arg(path_buf.clone())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        // TODO perhaps capture stderr here, in case it gives a hint
        // about the issue?
        return Err(EditError::EditorCommandError.into());
    }

    let new_text = fs::read_to_string(path_buf.clone())?;
    Ok(new_text)
}

fn find_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_x| {
        eprintln!("$EDITOR not set, defaulting to `vim`.");
        "vim".to_string()
    })
}
