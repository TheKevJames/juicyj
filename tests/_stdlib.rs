extern crate juicyj;

mod common;

use common::read_src_file;

macro_rules! feature_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("stdlib/java/{}.java", $case);
            let src: String = read_src_file(&filename);

            let lexer = juicyj::lexer::Lexer::new(&filename, &src);
            for token in lexer.clone().collect::<Vec<Result<_, _>>>() {
                match token {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Lexer Error");
                        println!("{}", e);
                        assert!(false);
                        std::process::exit(1);
                    },
                }
            }

            let mut parser = juicyj::parser::Parser::new(lexer);
            let parse_tree = match parser.get_tree() {
                Ok(pt) => pt,
                Err(e) => {
                    println!("Parser Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            };

            let mut weeder = juicyj::weeder::Weeder::new(&filename, &parse_tree);
            match weeder.verify(None) {
                Ok(_) => (),
                Err(e) => {
                    println!("Weeder Verification Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            }

            match juicyj::common::AST::new(&parse_tree) {
                Ok(_) => assert!(true),
                Err(e) => {
                    println!("AST Construction Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            };

            assert!(true);
        }
    )*
    }
}

feature_tests! {
    io_outputstream: "io/OutputStream",
    io_printstream: "io/PrintStream",
    io_serializable: "io/Serializable",
    lang_boolean: "lang/Boolean",
    lang_byte: "lang/Byte",
    lang_character: "lang/Character",
    lang_class: "lang/Class",
    lang_cloneable: "lang/Cloneable",
    lang_integer: "lang/Integer",
    lang_number: "lang/Number",
    lang_object: "lang/Object",
    lang_short: "lang/Short",
    lang_string: "lang/String",
    lang_system: "lang/System",
    util_arrays: "util/Arrays",
}
