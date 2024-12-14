use std::fs;

pub const UTIL_STR: &str = "utils";

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Should have been able to read the file")
}

pub fn read_file_to_vec(file_name: &str) -> Vec<String> {
    let content = String::from(read_file(file_name));
    let content_arr: Vec<&str> = content.split('\n').collect();
    let mut res = Vec::new();
    for i in content_arr {
        res.push(String::from(i));
    }
    res
}

pub fn write_vec_to_file(file_name: &str, content: &Vec<String>) {
    let _ = fs::write(file_name, content.join("\n"));
}
