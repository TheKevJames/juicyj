//! The analysis module for juicyj. Includes the environment builder. Roughly
//! corresponds to assignment 2 of the original CS444 project.
mod environment;

use std;

use scanner::AST;

use self::environment::Environment;

/// Runs a file through the analysis stack (environment builder) and exits with
/// code 42 on a failure.
///
/// # Examples
///
/// ```rust,no_run
/// let filename = "Sample.java";
/// let contents = juicyj::scanner::read_src_file(&filename.to_owned());
/// let ast = juicyj::scanner::scan_or_exit(&filename, &contents);
/// juicyj::analysis::analyze_or_exit(&vec![ast])
/// ```
pub fn analyze_or_exit(asts: &Vec<AST>) {
    match Environment::annotate_asts(asts) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };
}

// TODO: this should be #[cfg(test)], but for some reason the test macros can't
// find this module in that case.
#[allow(missing_docs)]
pub mod tests {
    use std;

    use super::super::scanner::AST;

    use super::environment::Environment;

    pub fn analyze_and_assert(asts: &Vec<AST>) {
        match Environment::annotate_asts(asts) {
            Ok(_) => (),
            Err(e) => {
                println!("Annotation Error");
                println!("{}", e);
                assert!(true);
                return;
            }
        };

        println!("No Error Found");
        assert!(false);
    }

    pub fn analyze_or_assert(asts: &Vec<AST>) {
        match Environment::annotate_asts(asts) {
            Ok(_) => (),
            Err(e) => {
                println!("Annotation Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        };
    }
}