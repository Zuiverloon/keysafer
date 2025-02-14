use image::{ ImageBuffer, Rgb };
use std::fs::File;
use std::io::{ self, Read, Write };

pub fn file_encrypt(input_file: &str, output_file: &str, key: u8) {
    println!("File From {} encrypted to {}", input_file, output_file);
    let mut file = File::open(input_file);
    let mut buffer = Vec::new();
    file.expect("REASON").read_to_end(&mut buffer);

    for i in 0..buffer.len() {
        buffer[i] ^= key;
    }

    // Write the binary content to a new file
    let mut output_file = File::create(output_file);
    output_file.expect("REASON").write_all(&buffer);
}
