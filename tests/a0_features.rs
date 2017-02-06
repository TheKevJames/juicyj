extern crate juicyj;

mod common;

use common::read_src_file;

macro_rules! feature_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("tests/cases/features/{}.java", $case);
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
    a01: "A1",
    a02: "A2",
    a03: "A3",
    a04: "A4",
    a05: "A5",
    a06: "A6",
    a07: "A7",
    a08: "A8",
    a09: "A9",
    a10: "A10",
    a11: "A11",
    a12: "A12",
    a13: "A13",
    a14: "A14",
    a15: "A15",
    a16: "A16",
    a17: "A17",
    a18: "A18",
    a19: "A19",
    a20: "A20",
    a21: "A21",
    a22: "A22",
    a23: "A23",
    a24: "A24",
    a25: "A25",
    a26: "A26",
    a27: "A27",
    a28: "A28",
    a29: "A29",
    a30: "A30",
    a31: "A31",
    a32: "A32",
    a33: "A33",
    a34: "A34",
    a35: "A35",
    a36: "A36",
    a37: "A37",
    a38: "A38",
    a39: "A39",
    a40: "A40",
    a41: "A41",
    a42: "A42",
    a43: "A43",
    a44: "A44",
    a45: "A45",
    a46: "A46",
    a47: "A47",
    a48: "A48",
    a49: "A49",
    a50: "A50",
    a51: "A51",
    a52: "A52",
    a53: "A53",
    a54: "A54",
    a55: "A55",
    a56: "A56",
    a57: "A57",
    a58: "A58",
    a59: "A59",
    a60: "A60",
}