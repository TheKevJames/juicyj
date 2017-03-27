//! The code generation module for juicyj. Roughly corresponds to assignment 5
//! of the original CS444 project.
extern crate walkdir;

mod body;

use std;
use std::fs;
use std::io::Write;

use analysis::ClassOrInterfaceEnvironment;
use analysis::Environment;

use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref INTEGER: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Int, None), children: Vec::new() }
    };
    static ref STATIC: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Static, None), children: Vec::new() }
    };
    static ref TEST: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Identifier, Some("test")), children: Vec::new() }
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

        for method in &self.methods {
            let label = match method.to_label(class_label.clone()) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            externs.push(format!("global {}", label));
            text.push(format!("{}:", label));

            // get args from stack
            text.push(format!("  ; get args"));
            for (idx, param) in method.parameters.iter().enumerate().rev() {
                let variable = match param.name.to_label() {
                    Ok(l) => l,
                    Err(e) => return Err(e),
                };
                bss.push(variable.clone());

                text.push(format!("  mov {}, {}", "esi", "esp"));
                text.push(format!("  add {}, {}", "esi", 4 * (idx + 1)));
                text.push(format!("  mov [{}], {}", variable, "esi"));
            }
            text.push("".to_owned());

            // TODO<codegen>: else error?
            if let Some(b) = method.body.clone() {
                match self::body::go(&b, &mut text, &mut externs, &mut bss, &mut data) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            if method.modifiers.contains(&*STATIC) && method.return_type == *INTEGER &&
               method.name == *TEST {
                // use this method as the entry point
                externs.push("global _start".to_owned());
                text.push("_start:".to_owned());

                text.push(format!("  push {}", "ebp"));
                text.push(format!("  mov {}, {}", "ebp", "esp"));
                text.push(format!("  call {}", label));
                text.push(format!("  mov {}, {}", "esp", "ebp"));
                text.push(format!("  pop {}", "ebp"));
                text.push("".to_owned());

                // exit with this method's return value
                text.push(format!("  mov {}, {}", "ebx", "eax"));
                text.push(format!("  mov {}, {}", "eax", "1"));
                text.push(format!("  int {}", "0x80"));
                text.push("".to_owned());
            }

            text.push("".to_owned());
        }

        let mut code = Vec::new();
        if !externs.is_empty() {
            externs.sort();
            externs.dedup();

            // do not import exported labels
            // iterate backward to ensure array deletion doesn't fuck with things
            for (idx, ext) in externs.clone().iter().enumerate().rev() {
                let split = ext.split_whitespace().collect::<Vec<&str>>();
                if split[0] != "extern" {
                    continue;
                }

                if externs.contains(&vec!["global", split[1]].join(" ")) {
                    externs.remove(idx);
                }
            }

            externs.insert(0, format!("section .text"));

            code.push(externs.join("\n"));
        }
        code.push(text.join("\n"));
        if !bss.is_empty() {
            bss.sort();
            bss.dedup();
            bss = bss.iter().map(|v| format!("  {}: resb {}", v, "32")).collect();
            bss.insert(0, format!("section .bss"));

            code.push(bss.join("\n"));
        }
        if !data.is_empty() {
            data.sort();
            data.dedup();
            data.insert(0, format!("section .data"));

            code.push(data.join("\n"));
        }
        Ok(code.join("\n\n"))
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
