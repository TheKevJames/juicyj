extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;

extern crate juicyj;

use std::env;

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("V", "version", "print the version");
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

    if matches.opt_present("V") {
        print_version(&program);
        return;
    }

    if matches.free.len() == 0 {
        print_usage(&program, opts);
        return;
    }

    for i in 0..matches.free.len() {
        let file = matches.free[i].clone();
        let src = juicyj::scanner::read_src_file(&file);
        juicyj::scanner::scan_or_exit(&file, &src);
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [-hV] FILE...", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} v{}", program, env!("CARGO_PKG_VERSION"));
}
