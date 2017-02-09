extern crate juicyj;

macro_rules! a4_pass_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let filename: String = format!("tests/cases/a4/pass/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

macro_rules! a4_pass_tests_folders {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let paths = std::fs::read_dir(format!("tests/cases/a4/pass/{}", $case)).unwrap();
            for path in paths {
                match path.unwrap().path().to_str() {
                    Some(filename) => {
                        // TODO: compile multiple together
                        let src: String = juicyj::scanner::read_src_file(&filename.to_string());
                        juicyj::scanner::tests::scan_or_assert(&filename, &src);
                    }
                    _ => (),
                }
            }
        }
    )*
    }
}

a4_pass_tests! {
    j1_7_reachability_afterifwithwhiletrue: "J1_7_Reachability_AfterIfWithWhileTrue",
    j1_7_reachability_ifthenelse_invaluemethod: "J1_7_Reachability_IfThenElse_InValueMethod",
    j1_arbitraryreturn: "J1_arbitraryreturn",
    j1_multiplereturn: "J1_multipleReturn",
    j1_reachable2: "J1_Reachable2",
    j1_unreachableautomation: "J1_unreachableAutomation",
    j1_whiletrue1: "J1_whiletrue1",
    j1_7_reachability_emptyvoidmethod: "J1_7_Reachability_EmptyVoidMethod",
    j1_7_reachability_ifthenelse_invoidmethod: "J1_7_Reachability_IfThenElse_InVoidMethod",
    j1_defasn_use_before_declare: "J1_defasn_use_before_declare",
    j1_omittedvoidreturn: "J1_omittedvoidreturn",
    j1_reachable3: "J1_Reachable3",
    j1_unreachable: "J1_Unreachable",
    j1_7_reachability_ifandwhile_return: "J1_7_Reachability_IfAndWhile_Return",
    j1_7_reachability_whiletrue_constantfolding: "J1_7_Reachability_WhileTrue_ConstantFolding",
    j1_ifthenelse: "J1_ifThenElse",
    j1_reachable4: "J1_Reachable4",
    j1_while1: "J1_while1",
    j1_7_reachability_ifthenelse_inconstructor: "J1_7_Reachability_IfThenElse_InConstructor",
    j1_7_unreachable_ifequalsnot: "J1_7_Unreachable_IfEqualsNot",
    j1_if_then: "J1_if_then",
    j1_reachable1: "J1_Reachable1",
    j1_reachableifbody: "J1_reachableIfBody",
    j1_while2: "J1_while2",
}

a4_pass_tests_folders! {
    j1_reachability_return: "J1_reachability_return",
}
