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
    opts.optflag("s", "stdlib", "include stdlib in compilation");
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

    let mut asts = Vec::new();
    for i in 0..matches.free.len() {
        let file = matches.free[i].clone();
        let src = juicyj::scanner::read_src_file(&file);
        asts.push(juicyj::scanner::scan_or_exit(&file, &src));
    }

    if matches.opt_present("s") {
        let stdlib_io = std::fs::read_dir("stdlib/java/io").unwrap();
        let stdlib_lang = std::fs::read_dir("stdlib/java/lang").unwrap();
        let stdlib_util = std::fs::read_dir("stdlib/java/util").unwrap();

        for path in stdlib_io.chain(stdlib_lang).chain(stdlib_util) {
            match path.unwrap().path().to_str() {
                Some(filename) => {
                    let src: String = juicyj::scanner::read_src_file(&filename.to_string());
                    asts.push(juicyj::scanner::scan_or_exit(&filename, &src));
                }
                _ => (),
            }
        }
    }

    let env = juicyj::analysis::analyze_or_exit(&asts);
    juicyj::generator::generate_or_exit(&env);
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] FILE...", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} v{}", program, env!("CARGO_PKG_VERSION"));
}
