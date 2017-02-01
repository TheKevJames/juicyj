use std::fs::File;
use std::io::Read;

// See main.rs
pub fn read_src_file(file: &String) -> String {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => panic!("could not open file: {}", file),
    };

    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Ok(_) => {}
        Err(_) => panic!("could not read file to string"),
    };

    src
}
