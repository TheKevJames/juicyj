extern crate env_logger;
#[macro_use]
extern crate log;

extern crate juicyj;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    env_logger::init().unwrap();

    if env::args().count() != 2 {
        error!("usage: {} <src_file>", env::args().nth(0).unwrap());
        std::process::exit(1);
    }

    let src_file = env::args().nth(1).unwrap();
    debug!("Using src file {}", src_file);

    let mut file = match File::open(&src_file) {
        Ok(file) => file,
        Err(_) => {
            error!("could not open file: {}", src_file);
            std::process::exit(1);
        }
    };

    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Ok(_) => {}
        Err(_) => {
            error!("could not read file to string");
            std::process::exit(1);
        }
    };

    let lexer = juicyj::lexer::Lexer::new(&src);
    let tokens = lexer.tokenize();
    debug!("got tokens {:?}", tokens);
}
