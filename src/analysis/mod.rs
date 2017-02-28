mod environment;

use std;

use scanner::AST;

use self::environment::Environment;

pub fn analyze_or_exit(asts: Vec<AST>) {
    let annotated = match Environment::annotate_asts(asts) {
        Ok(a) => a,
        Err(e) => {
            print!("{}", e);
            std::process::exit(42);
        }
    };

    for anno in annotated {
        println!("{}", anno);
    }
}
