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
    assert_eq!(tokens.len(), 708);
}

#[test]
fn test_all_cases() {
    let paths = std::fs::read_dir("tests/cases/").unwrap();
    for path in paths {
        match path.unwrap().path().to_str() {
            Some(name) => {
                let src = read_src_file(String::from(name));
                let lexer = juicyj::lexer::Lexer::new(&src);
                lexer.collect::<Vec<juicyj::common::Token>>();

                // TODO: consider validating token values
            }
            _ => continue,
        }
    }
}
