extern crate juicyj;

use std::fs::File;
use std::io::Read;

// See main.rs
fn read_src_file(file: String) -> String {
    let mut file = match File::open(&file) {
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

#[test]
fn test_valid_language_features() {
    let src = read_src_file(String::from("tests/language_features.joos"));
    let lexer = juicyj::lexer::Lexer::new(&src);
    let tokens = lexer.collect::<Vec<juicyj::common::Token>>();
    // Yeah, I counted.
    assert!(tokens.len() == 684, "got {} tokens", tokens.len());
}
