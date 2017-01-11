extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;

extern crate juicyj;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let file = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let src = read_src_file(file);

    let lexer = juicyj::lexer::Lexer::new(&src);
    let tokens = lexer.tokenize();
    debug!("got tokens {:?}", tokens);
}

fn read_src_file(file: String) -> String {
    debug!("Using src file {}", file);

    let mut file = match File::open(&file) {
        Ok(file) => file,
        Err(_) => {
            error!("could not open file: {}", file);
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

    src
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
