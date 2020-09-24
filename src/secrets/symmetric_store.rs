use std::error;

use tindercrypt::cryptors::RingCryptor;

use crate::secrets;

pub fn encrypt(input: String) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let plaintext = input.as_bytes();
    let pass = secrets::keychain_access::get_symmetric_key();
    let cryptor = RingCryptor::new();

    Ok(cryptor.seal_with_passphrase(pass?.as_bytes(), plaintext)?)
}

pub fn decrypt(input: Vec<u8>) -> Result<String, Box<dyn error::Error>> {
    let pass = secrets::keychain_access::get_symmetric_key();
    let cryptor = RingCryptor::new();

    let plaintext = cryptor.open(pass?.as_bytes(), &input)?;
    Ok(String::from_utf8(plaintext)?)
}
