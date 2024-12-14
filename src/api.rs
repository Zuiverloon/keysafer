use crate::secret_utils;
use crate::crypt_utils;
use crate::file_utils;
use crate::constant;

pub fn get_secret() -> String {
    secret_utils::get_secret()
}

pub fn get_password() -> String {
    let mut passwords = file_utils::read_file_to_vec(&constant::get_plaintext_password_dir());
    passwords.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase()));
    passwords.join("\n")
}

pub fn encrypt(data: &str) -> String {
    crypt_utils::encrypt(data)
}

pub fn decrypt(data: &str) -> String {
    crypt_utils::decrypt(data)
}

pub fn password_encrypt() {
    let lines = file_utils::read_file_to_vec(&constant::get_plaintext_password_dir());
    // println!("{:?}", lines);
    let encrypted_lines: Vec<String> = lines
        .iter()
        .map(|line| encrypt_line(line))
        .collect();
    // println!("{:?}", encrypted_lines);
    file_utils::write_vec_to_file(&constant::get_encrypted_password_dir(), &encrypted_lines);
}

pub fn password_recover() {
    let lines = file_utils::read_file_to_vec(&constant::get_encrypted_password_dir());
    // println!("{:?}", lines);
    let decrypted_lines: Vec<String> = lines
        .iter()
        .map(|line| decrypt_line(line))
        .collect();
    // println!("{:?}", decrypted_lines);
    file_utils::write_vec_to_file(&constant::get_plaintext_password_dir(), &decrypted_lines);
}

pub fn mnemonic_encrypt() {
    constant::MNEMONIC_LIST.iter().for_each(|mnemonic| process_encrypt_mnemonic(mnemonic))
}

pub fn mnemonic_recover() {
    constant::MNEMONIC_LIST.iter().for_each(|mnemonic| process_recover_mnemonic(mnemonic))
}

pub fn get_mnemonic(id: &str) -> String {
    match constant::MNEMONIC_LIST.iter().find(|&&x| x == id.to_uppercase()) {
        Some(x) => read_mnemonics_file(*x),
        _ => String::from("Invalid mnemonic id"),
    }
}

// pub fn mnemonic_recover() {
//     constant::MNEMONIC_LIST.iter().for_each(|mnemonic| process_recover_mnemonic(mnemonic))
// }

fn encrypt_line(line: &str) -> String {
    line.split(" ")
        .map(|word| crypt_utils::encrypt(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn decrypt_line(line: &str) -> String {
    line.split(" ")
        .map(|word| crypt_utils::decrypt(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn process_encrypt_mnemonic(name: &str) {
    let plaintext_file_name = constant::get_plaintext_mnemonic_dir(name);
    let encrypted_file_name = constant::get_encrypted_mnemonic_dir(name);
    println!("{}", plaintext_file_name);
    let plaintext_mnemonic_tokens = file_utils::read_file_to_vec(&plaintext_file_name);
    println!("{:?}", plaintext_mnemonic_tokens);
    let encrypted_mnemonic_tokens: Vec<_> = plaintext_mnemonic_tokens
        .iter()
        .map(|token| crypt_utils::encrypt(token))
        .collect::<Vec<_>>();
    file_utils::write_vec_to_file(&encrypted_file_name, &encrypted_mnemonic_tokens);

    // let decrypted_mnemonic_tokens: Vec<_> = encrypted_mnemonic_tokens
    //     .iter()
    //     .map(|token| crypt_utils::decrypt(token))
    //     .collect::<Vec<_>>();
    // println!("{:?}", decrypted_mnemonic_tokens);
}

fn process_recover_mnemonic(name: &str) {
    let plaintext_file_name = constant::get_plaintext_mnemonic_dir(name);
    let encrypted_file_name = constant::get_encrypted_mnemonic_dir(name);
    // println!("{}", plaintext_file_name);
    let encrypted_mnemonic_tokens = file_utils::read_file_to_vec(&encrypted_file_name);
    // println!("{:?}", plaintext_mnemonic_tokens);
    let plaintext_mnemonic_tokens: Vec<_> = encrypted_mnemonic_tokens
        .iter()
        .map(|token| crypt_utils::decrypt(token))
        .collect::<Vec<_>>();
    file_utils::write_vec_to_file(&plaintext_file_name, &plaintext_mnemonic_tokens);

    // let decrypted_mnemonic_tokens: Vec<_> = encrypted_mnemonic_tokens
    //     .iter()
    //     .map(|token| crypt_utils::decrypt(token))
    //     .collect::<Vec<_>>();

    // println!("{:?}", decrypted_mnemonic_tokens);
}

fn read_mnemonics_file(id: &str) -> String {
    file_utils::read_file_to_vec(&constant::get_plaintext_mnemonic_dir(id)).join(" ")
}
