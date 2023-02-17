use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

pub fn encrypt(plaintext: &str, key: &[u8], iv: &[u8]) -> String {
    let cipher = Cbc::<Aes256, Pkcs7>::new_var(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());
    base64::encode(&ciphertext)
}

pub fn decrypt(ciphertext: &str, key: &[u8], iv: &[u8]) -> String {
    let cipher = Cbc::<Aes256, Pkcs7>::new_var(key, iv).unwrap();
    let ciphertext = base64::decode(ciphertext).unwrap();
    let plaintext = cipher.decrypt_vec(&ciphertext).unwrap();
    String::from_utf8(plaintext).unwrap()
}