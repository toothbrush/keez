use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::process::Command;
use std::process::Stdio;

use mktemp::Temp;

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
    let yaml_blob = serde_yaml::to_string(&params)?;

    let new_yaml_blob = interactive_edit(yaml_blob).unwrap();

    if debug {
        eprintln!("New YAML blob after edit session:");
        eprintln!("{}", new_yaml_blob);
    }
    // TODO re-open editor if something about the new YAML makes it
    // unparsable, or if something goes wrong pushing to AWS API.

    // Deserialize it back to a Rust type.
    let deserialized: aws::parameter_store::ParameterCollection =
        serde_yaml::from_str(&new_yaml_blob)?;

    if debug {
        eprintln!("Data structure after deserialization:");
        eprintln!("{:?}", deserialized);
    }

    Ok(deserialized)
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
