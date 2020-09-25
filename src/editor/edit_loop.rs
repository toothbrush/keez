use std::error;
use std::fmt;
use std::fs;
use std::process::Command;
use std::process::Stdio;

use mktemp::Temp;

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

pub fn interactive_edit(text: String) -> Result<String, Box<dyn error::Error>> {
    let editor = match find_editor() {
        Ok(e) => e,
        Err(err) => return Err(err),
    };

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

fn find_editor() -> Result<String, Box<dyn error::Error>> {
    // TODO use env::... and $EDITOR, fall back to Vim.  Or something
    // smarter.
    Ok("vim".to_string())
}
