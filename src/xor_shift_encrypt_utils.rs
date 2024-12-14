pub fn xor_cipher(plaintext: &str, key: &str) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();

    let mut ciphered_data = Vec::new();

    for (i, byte) in plaintext_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        ciphered_data.push(byte ^ key_byte);
    }

    ciphered_data
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub fn xor_decipher(ciphertext_hex: &str, key: &str) -> String {
    let ciphertext_bytes = (0..ciphertext_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&ciphertext_hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();

    let key_bytes = key.as_bytes();

    let mut deciphered_data = Vec::new();

    for (i, byte) in ciphertext_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        deciphered_data.push(byte ^ key_byte);
    }

    String::from_utf8(deciphered_data).unwrap_or_else(|_| "Invalid UTF-8 sequence".to_string())
}
