use crate::file_utils;
use crate::constant;

pub fn get_secret() -> String {
    file_utils::read_file(&constant::get_secret_dir())
}
