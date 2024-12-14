use crate::secret_utils;
use crate::xor_shift_encrypt_utils;

fn get_secret() -> String {
    secret_utils::get_secret()
}

pub fn encrypt(data: &str) -> String {
    xor_shift_encrypt_utils::xor_cipher(data, &get_secret())
}

pub fn decrypt(data: &str) -> String {
    xor_shift_encrypt_utils::xor_decipher(data, &get_secret())
}
