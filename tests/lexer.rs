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
    let filename = "tests/language_features.joos";

    let src = read_src_file(String::from(filename));
    let lexer = juicyj::lexer::Lexer::new(&filename, &src);
    let tokens = lexer.map(|t| {
            if t.is_err() {
                println!("{}", t.err().unwrap());
                assert!(false);
                return None;
            }
            t.ok()
        })
        .collect::<Vec<Option<juicyj::common::Token>>>();
    // Yeah, I counted.
    assert_eq!(tokens.len(), 714);
}

// TODO: multiple test cases
#[test]
fn test_all_cases() {
    let paths = std::fs::read_dir("tests/cases/").unwrap();
    for path in paths {
        match path.unwrap().path().to_str() {
            Some(name) => {
                let src = read_src_file(String::from(name));
                let lexer = juicyj::lexer::Lexer::new(&name, &src);

                let errors = lexer.filter(|result| result.is_err()).collect::<Vec<_>>();
                let errored = !errors.is_empty();

                if name.starts_with("tests/cases/Je") {
                    if !errored {
                        println!("failed test: {}", name);
                    }
                    // TODO: most of these require a working parser to error
                    // assert!(errored);
                } else {
                    for err in errors {
                        println!("{}", err.err().unwrap());
                    }
                    assert!(!errored);
                }
            }
            _ => continue,
        }
    }
    // Until the above TODO is fixed, uncomment this to see all failing tests
    // assert!(false);
}
