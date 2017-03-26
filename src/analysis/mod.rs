//! The analysis module for juicyj. Includes the environment builder. Roughly
//! corresponds to assignments 2 through 4 of the original CS444 project.
mod environment;
mod types;

use std;

use scanner::AST;

pub use self::environment::ClassOrInterfaceEnvironment;
pub use self::environment::Environment;
use self::types::verify;

/// Runs a set of ASTs through the analysis stack (environment builder) and
/// exits with code 42 on a failure.
///
/// # Examples
///
/// ```rust,no_run
/// let filename = "Sample.java";
/// let contents = juicyj::scanner::read_src_file(&filename.to_owned());
/// let ast = juicyj::scanner::scan_or_exit(&filename, &contents);
/// juicyj::analysis::analyze_or_exit(&vec![ast])
/// ```
pub fn analyze_or_exit(asts: &Vec<AST>) -> Environment {
    let env = match Environment::new(asts) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };

    match verify(&env) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    }

    env
}

// TODO: this should be #[cfg(test)], but for some reason the test macros can't
// find this module in that case.
#[allow(missing_docs)]
pub mod tests {
    use std;

    use super::super::scanner::AST;

    use super::environment::Environment;
    use super::types::verify;

    pub fn analyze_and_assert(asts: &Vec<AST>) {
        let env = match Environment::new(asts) {
            Ok(e) => e,
            Err(e) => {
                println!("Annotation Error");
                println!("{}", e);
                assert!(true);
                return;
            }
        };

        match verify(&env) {
            Ok(_) => (),
            Err(e) => {
                println!("Verification Error");
                println!("{}", e);
                assert!(true);
                return;
            }
        }

        println!("No Error Found");
        assert!(false);
    }

    pub fn analyze_or_assert(asts: &Vec<AST>) {
        let env = match Environment::new(asts) {
            Ok(e) => e,
            Err(e) => {
                println!("Annotation Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        };

        match verify(&env) {
            Ok(_) => (),
            Err(e) => {
                println!("Verification Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        }
    }
}
