//! The code generation module for juicyj. Roughly corresponds to assignment 5
//! of the original CS444 project.
extern crate walkdir;

use std;
use std::fs;
use std::io::Write;

use scanner::AST;

/// Runs a set of ASTs through code generation and exits with code 42 on a
/// failure. If no failure exists, the compiled files will be located in the
/// `output/` subdirectory in the current working directory.
pub fn generate_or_exit(asts: &Vec<AST>) {
    match fs::create_dir_all("output") {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };

    for path in walkdir::WalkDir::new("output") {
        match path.unwrap().path().to_str() {
            Some(filename) if filename != "output" => {
                if fs::remove_file(filename).is_err() {
                    println!("Could not remove file 'output/{}'", filename);
                    std::process::exit(42);
                }
            }
            _ => (),
        }
    }

    for ast in asts {
        let name = ast.canonical
            .children
            .iter()
            .map(|n| n.clone().token.lexeme.unwrap_or("".to_owned()).to_lowercase())
            .collect::<Vec<String>>()
            .join("");

        let mut f = match fs::File::create(format!("output/{}.s", name)) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        };

        let data = "nop".as_bytes();
        match f.write_all(data) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        }
    }
}
