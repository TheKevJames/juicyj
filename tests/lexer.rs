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

                let mut errors =
                    lexer.map(|result| { if result.is_err() { result.err() } else { None } });
                let errored = errors.any(|x| x.is_some());
                // let errored = lexer.fold(false, |acc, result| acc ^ result.is_err());

                // TODO: hangs
                // if name.starts_with("Je") {
                if name.contains("Je") {
                    if !errored {
                        println!("failed test: {}", name);
                    }
                    // TODO: most of these require a working parser to error
                    // assert!(errored);
                } else {
                    if errored {
                        errors.map(|err| {
                                if err.is_some() {
                                    println!("{}", err.unwrap());
                                }
                            })
                            .collect::<Vec<_>>();
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
