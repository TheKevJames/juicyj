//! The code generation module for juicyj. Roughly corresponds to assignment 5
//! of the original CS444 project.
extern crate walkdir;

mod asm;
mod body;
mod class;

use std;
use std::fs;
use std::io::Write;

use analysis::ClassOrInterfaceEnvironment;
use analysis::Environment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
    };
}

trait Generatable {
    fn generate(&self) -> Result<String, String>;
}

impl Generatable for ClassOrInterfaceEnvironment {
    fn generate(&self) -> Result<String, String> {
        let class_label = match self.name.to_label() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let mut bss: Vec<String> = Vec::new();
        let mut data: Vec<String> = Vec::new();
        let mut externs: Vec<String> = Vec::new();
        let mut text: Vec<String> = Vec::new();

        // externs.push(format!("extern {}", "__exception"));
        // externs.push(format!("extern {}", "__NATIVEjava.io.OutputStream.nativeWrite"));

        let mut init_fields = Vec::new();
        for field in &self.fields {
            let mut name = self.name.clone();
            name.flatten();
            name.children.push(DOT.clone());
            name.children.push(field.name.clone());

            let mut field = field.clone();
            field.name = name;

            let label = match field.name.to_label() {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            match class::field::go(&field, &label, &mut text, &mut externs, &mut bss, &mut data) {
                Ok(Some(n)) => init_fields.push((label.clone(), n.clone())),
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for constructor in &self.constructors {
            let label = match class::method::get_label(constructor,
                                                       &class_label,
                                                       &mut text,
                                                       &mut externs) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            match class::constructor::go(&constructor,
                                         &label,
                                         &init_fields,
                                         &mut text,
                                         &mut externs,
                                         &mut bss,
                                         &mut data) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for method in &self.methods {
            let label =
                match class::method::get_label(method, &class_label, &mut text, &mut externs) {
                    Ok(l) => l,
                    Err(e) => return Err(e),
                };
            match class::method::go(&method,
                                    &label,
                                    &mut text,
                                    &mut externs,
                                    &mut bss,
                                    &mut data) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(class::code(&text, &externs, &bss, &data))
    }
}

/// Runs an Environment through code generation and exits with code 42 on a
/// failure. If no failure exists, the compiled files will be located in the
/// `output/` subdirectory in the current working directory.
pub fn generate_or_exit(env: &Environment) {
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

    for kind in &env.kinds {
        let name = kind.name
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

        let source = match kind.generate() {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        };

        match f.write_all(source.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        }
    }
}
