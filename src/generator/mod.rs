//! The code generation module for juicyj. Roughly corresponds to assignment 5
//! of the original CS444 project.
extern crate walkdir;

mod body;

use std;
use std::fs;
use std::io::Write;

use analysis::ClassOrInterfaceEnvironment;
use analysis::Environment;

trait Generatable {
    fn generate(&self) -> String;
}

impl Generatable for ClassOrInterfaceEnvironment {
    fn generate(&self) -> String {
        let class_label = self.name.to_label();
        let mut bss: Vec<String> = Vec::new();
        let mut data: Vec<String> = Vec::new();
        let mut text: Vec<String> = Vec::new();
        let mut textpre: Vec<String> = Vec::new();

        textpre.push(format!("section .text"));
        textpre.push(format!("extern {}", "__exception"));
        textpre.push(format!("extern {}", "__malloc"));
        textpre.push(format!("extern {}", "__NATIVEjava.io.OutputStream.nativeWrite"));

        for method in &self.methods {
            let label = method.to_label(class_label.clone());
            textpre.push(format!("global _{}", label));
            text.push(format!("_{}:", label));

            if let Some(b) = method.body.clone() {
                self::body::go(&b, &mut text, &mut bss, &mut data);
            }
            // TODO: else error?

            if label == "start" {
                // exit with this method's return value
                text.push(format!("  mov {}, {}", "ebx", "eax"));
                text.push(format!("  mov {}, {}", "eax", "1"));
                text.push(format!("  int {}", "0x80"));
            }

            text.push("".to_owned());
        }

        let mut code = Vec::new();
        code.push(textpre.join("\n"));
        code.push(text.join("\n"));
        if !bss.is_empty() {
            bss.sort();
            bss.dedup();
            bss.insert(0, format!("section .bss"));

            code.push(bss.join("\n"));
        }
        if !data.is_empty() {
            data.sort();
            data.dedup();
            data.insert(0, format!("section .data"));

            code.push(data.join("\n"));
        }
        code.join("\n\n")
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

        match f.write_all(kind.generate().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        }
    }
}
