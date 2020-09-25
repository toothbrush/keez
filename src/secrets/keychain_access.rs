use keyring::Keyring;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

const APP_NAME: &str = "keez";
const SYMMETRIC_KEY_ID: &str = "temporary symmetric key";

/// This function will return the stored key intended for symmetric
/// encryption of exported parameter values, and if it doesn't yet
/// exist (e.g., on first invocation) will generate a key and store
/// it, too.
pub fn get_symmetric_key() -> Result<String, keyring::KeyringError> {
    let keyring = Keyring::new(APP_NAME, SYMMETRIC_KEY_ID);
    let get_password_result = keyring.get_password();

    let debug = false; // TODO proper config management
    if debug {
        eprintln!("{:?}", get_password_result);
    }

    // Only if the error was "not found" do we know how to recover.
    // Otherwise, bubble up the error.
    match get_password_result {
        Ok(item) => return Ok(item),
        Err(keyring::KeyringError::NoPasswordFound) => {
            return set_symmetric_key();
        }
        Err(other) => return Err(other),
    }
}

pub fn set_symmetric_key() -> Result<String, keyring::KeyringError> {
    let keyring = Keyring::new(APP_NAME, SYMMETRIC_KEY_ID);

    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(128).collect();

    let set_password_result = keyring.set_password(&rand_string);

    match set_password_result {
        Ok(()) => return Ok(rand_string),
        Err(whatever) => return Err(whatever),
    }
}
