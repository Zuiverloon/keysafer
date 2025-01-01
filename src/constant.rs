const PLAINTEXT_BASE: &str = "src/tokens/";
const ENCRYPTED_BASE: &str = "src/encrypted_tokens/";

const SECRET_FILE: &str = "secret.txt";
const PASSWORD_FILE: &str = "password.txt";

pub const MNEMONIC_LIST: [&str; 23] = [
    "BITGET_LITE",
    "BITGET", //0x915C...975A69
    "ECOCHAINLESS",
    "ERA",
    "FUEL",
    "GIT",
    "MEMEFI",
    "METAMASK", //0x5f76...5f8e6D
    "PHANTOM", // 0xa9aa...67C2b2
    "PRE0203",
    "PRE0221",
    "PRE0603",
    "PRE0831",
    "PRE0901",
    "PRE0924",
    "PRE1121",
    "PRE4201",
    "RAZOR",
    "SUBWALLET",
    "TON", // UQBL...rVSE
    "UNISAT",
    "VENOM",
    "MANGO",
];

pub fn get_secret_dir() -> String {
    format!("{}{}", PLAINTEXT_BASE, SECRET_FILE)
}

pub fn get_plaintext_password_dir() -> String {
    format!("{}{}", PLAINTEXT_BASE, PASSWORD_FILE)
}
pub fn get_encrypted_password_dir() -> String {
    format!("{}{}", ENCRYPTED_BASE, PASSWORD_FILE)
}

pub fn get_plaintext_password_dir_test() -> String {
    format!("{}{}_test", PLAINTEXT_BASE, PASSWORD_FILE)
}
pub fn get_encrypted_password_dir_test() -> String {
    format!("{}{}_test", ENCRYPTED_BASE, PASSWORD_FILE)
}

pub fn get_plaintext_mnemonic_dir(name: &str) -> String {
    get_plaintext_dir(&format!("{}.txt", name))
}

pub fn get_encrypted_mnemonic_dir(name: &str) -> String {
    get_encrypted_dir(&format!("{}.txt", name))
}

fn get_plaintext_dir(name: &str) -> String {
    format!("{}{}", PLAINTEXT_BASE, name)
}
fn get_encrypted_dir(name: &str) -> String {
    format!("{}{}", ENCRYPTED_BASE, name)
}
