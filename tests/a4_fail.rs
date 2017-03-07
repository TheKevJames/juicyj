extern crate juicyj;
extern crate walkdir;

macro_rules! a4_fail_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let stdlib_io = std::fs::read_dir("stdlib/java/io").unwrap();
            let stdlib_lang = std::fs::read_dir("stdlib/java/lang").unwrap();
            let stdlib_util = std::fs::read_dir("stdlib/java/util").unwrap();

            let mut asts = Vec::new();

            for path in stdlib_io.chain(stdlib_lang).chain(stdlib_util) {
                match path.unwrap().path().to_str() {
                    Some(filename) => {
                        let src: String = juicyj::scanner::read_src_file(&filename.to_string());
                        asts.push(juicyj::scanner::tests::scan_or_assert(&filename, &src));
                    }
                    _ => (),
                }
            }

            let filename: String = format!("tests/cases/a4/fail/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);
            asts.push(juicyj::scanner::tests::scan_or_assert(&filename, &src));

            juicyj::analysis::tests::analyze_and_assert(&asts);
        }
    )*
    }
}

a4_fail_tests! {
    je_7_definiteassignment_2lazyor_assignment: "Je_7_DefiniteAssignment_2LazyOr_Assignment",
    je_7_reachability_emptyvaluemethod: "Je_7_Reachability_EmptyValueMethod",
    je_7_return_ifelseif: "Je_7_Return_IfElseIf",
    je_8_definiteassignment_ififnot: "Je_8_DefiniteAssignment_IfIfNot",
    je_7_definiteassignment_3lazyor_assignment: "Je_7_DefiniteAssignment_3LazyOr_Assignment",
    je_7_reachability_forfalse_1: "Je_7_Reachability_ForFalse_1",
    je_7_return_ififnoelseelse: "Je_7_Return_IfIfNoElseElse",
    je_8_definiteassignment_inittoitself: "Je_8_DefiniteAssignment_InitToItself",
    je_7_reachability_afterelsereturn: "Je_7_Reachability_AfterElseReturn",
    je_7_reachability_forfalse_2: "Je_7_Reachability_ForFalse_2",
    je_7_return_ififnot: "Je_7_Return_IfIfNot",
    je_8_definiteassignment_somethingandassignment:
        "Je_8_DefiniteAssignment_SomethingAndAssignment",
    je_7_reachability_afterifreturnelsereturn: "Je_7_Reachability_AfterIfReturnElseReturn",
    je_7_reachability_returnreturn: "Je_7_Reachability_ReturnReturn",
    je_7_return_missinginelse: "Je_7_Return_MissingInElse",
    je_8_definiteassignment_somethingorassignment: "Je_8_DefiniteAssignment_SomethingOrAssignment",
    je_7_reachability_afterifreturn: "Je_7_Reachability_AfterIfReturn",
    je_7_reachability_whilefalse_constantfolding: "Je_7_Reachability_WhileFalse_ConstantFolding",
    je_8_definiteassignment_arrayassign: "Je_8_DefiniteAssignment_ArrayAssign",
    je_8_definiteassignment_uninitializedexpinlvalue:
        "Je_8_DefiniteAssignment_UninitializedExpInLvalue",
    je_7_reachability_afterreturn_constructor: "Je_7_Reachability_AfterReturn_Constructor",
    je_7_reachability_whilefalse_empty: "Je_7_Reachability_WhileFalse_Empty",
    je_8_definiteassignment_arrayindexassign: "Je_8_DefiniteAssignment_ArrayIndexAssign",
    je_8_definiteassignment_uninitializedinnewarray:
        "Je_8_DefiniteAssignment_UninitializedInNewArray",
    je_7_reachability_afterreturnemptyblock: "Je_7_Reachability_AfterReturnEmptyBlock",
    je_7_reachability_whilefalse_ifthenelse: "Je_7_Reachability_WhileFalse_IfThenElse",
    je_8_definiteassignment_complexinitializer: "Je_8_DefiniteAssignment_ComplexInitializer",
    je_8_definiteassignment_whilefalse: "Je_8_DefiniteAssignment_WhileFalse",
    je_7_reachability_aftervaluereturn: "Je_7_Reachability_AfterValueReturn",
    je_7_reachability_whiletrue_constantfolding: "Je_7_Reachability_WhileTrue_ConstantFolding",
    je_8_definiteassignment_falseandassignment: "Je_8_DefiniteAssignment_FalseAndAssignment",
    je_widening: "Je_Widening",
    je_7_reachability_aftervoidreturn: "Je_7_Reachability_AfterVoidReturn",
    je_7_reachability_whiletrue: "Je_7_Reachability_WhileTrue",
    je_8_definiteassignment_fieldwithsamename: "Je_8_DefiniteAssignment_FieldWithSameName",
}
