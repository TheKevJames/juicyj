mod environment;

use std;

use scanner::AST;

use self::environment::Environment;

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
