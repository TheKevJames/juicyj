extern crate juicyj;

mod common;

use common::read_src_file;

macro_rules! public_succeed_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("tests/cases/public_succeed/{}.java", $case);
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
                Ok(pt) => Ok(pt),
                Err(e) => {
                    println!("Parser Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            };

            let mut weeder = match juicyj::weeder::Weeder::new(&filename, &parse_tree) {
                Ok(w) => w,
                Err(e) => {
                    println!("Weeder Construction Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            };
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
        }
    )*
    }
}

public_succeed_tests! {
    j1_01: "J1_01",
    j1_1_ambiguousname_accessresultfrommethod: "J1_1_AmbiguousName_AccessResultFromMethod",
    j1_1_cast_complement: "J1_1_Cast_Complement",
    j1_1_cast_multiplecastofsamevalue_1: "J1_1_Cast_MultipleCastOfSameValue_1",
    j1_1_cast_multiplecastofsamevalue_2: "J1_1_Cast_MultipleCastOfSameValue_2",
    j1_1_cast_multiplecastofsamevalue_3: "J1_1_Cast_MultipleCastOfSameValue_3",
    j1_1_cast_multiplereferencearray: "J1_1_Cast_MultipleReferenceArray",
    j1_1_escapes_3digitoctalanddigit: "J1_1_Escapes_3DigitOctalAndDigit",
    j1_1_instanceof_inlazyexp: "J1_1_Instanceof_InLazyExp",
    j1_1_instanceof_ofadditiveexpression: "J1_1_Instanceof_OfAdditiveExpression",
    j1_1_instanceof_ofcastexpression: "J1_1_Instanceof_OfCastExpression",
    j1_1_intrange_negativeint: "J1_1_IntRange_NegativeInt",
    j1_abstractclass: "J1_abstractclass",
    j1_abstractmethodwithoutbody: "J1_abstractmethodwithoutbody",
    j1_arbitrarylocaldeclaration: "J1_arbitrarylocaldeclaration",
    j1_arithmeticoperations: "J1_arithmeticoperations",
    j1_arraycreateandindex: "J1_ArrayCreateAndIndex",
    j1_assignment: "J1_assignment",
    j1_assignmentexp: "J1_assignmentExp",
    j1_barminusfoo: "J1_barminusfoo",
    j1_bigint: "J1_BigInt",
    j1_char: "J1_char",
    j1_char_escape: "J1_char_escape",
    j1_char_escape2: "J1_char_escape2",
    j1_char_escape3: "J1_char_escape3",
    j1_charadd: "J1_charadd",
    j1_charcast: "J1_CharCast",
    j1_charcharinit1: "J1_CharCharInit1",
    j1_charcharinit2: "J1_CharCharInit2",
    j1_charliterals: "J1_charliterals",
    j1_classinstance: "J1_classinstance",
    j1_commentsinexp1: "J1_commentsInExp1",
    j1_commentsinexp2: "J1_commentsInExp2",
    j1_commentsinexp3: "J1_commentsInExp3",
    j1_commentsinexp4: "J1_commentsInExp4",
    j1_commentsinexp5: "J1_commentsInExp5",
    j1_commentsinexp6: "J1_commentsInExp6",
    j1_commentsinexp7: "J1_commentsInExp7",
    j1_commentsinexp8: "J1_commentsInExp8",
    j1_commentsinexp9: "J1_commentsInExp9",
    j1_comparisonoperations: "J1_comparisonoperations",
    j1_concat_in_binop: "J1_concat_in_binop",
    j1_constructorbodycast: "J1_constructorbodycast",
    j1_constructorparameter: "J1_constructorparameter",
    j1_constructorwithsamenameasmethod: "J1_constructorWithSameNameAsMethod",
    j1_eagerbooleanoperations: "J1_eagerbooleanoperations",
    j1_escapeescape: "J1_EscapeEscape",
    j1_evalmethodinvocationfromparexp: "J1_evalMethodInvocationFromParExp",
    j1_exp: "J1_exp",
    j1_extends: "J1_extends",
    j1_externalcall: "J1_externalcall",
    j1_finalclass: "J1_finalclass",
    j1_finalclass2: "J1_finalclass2",
    j1_for_no_short_if: "J1_for_no_short_if",
    j1_forallwaysinit: "J1_forAllwaysInit",
    j1_foralwaysinitaswhile: "J1_forAlwaysInitAsWhile",
    j1_forbodycast: "J1_forbodycast",
    j1_forifstatements1: "J1_forifstatements1",
    j1_forifstatements2: "J1_forifstatements2",
    j1_forifstatements3: "J1_forifstatements3",
    j1_forinfor: "J1_forinfor",
    j1_forinitcast: "J1_forinitcast",
    j1_formethodinit: "J1_forMethodInit",
    j1_formethodupdate: "J1_forMethodUpdate",
    j1_formethodupdate2: "J1_forMethodUpdate2",
    j1_forupdate_classcreation: "J1_ForUpdate_ClassCreation",
    j1_forupdatecast: "J1_forupdatecast",
    j1_forwithoutexp: "J1_forWithoutExp",
    j1_forwithoutinit: "J1_forWithoutInit",
    j1_forwithoutupdate: "J1_forWithoutUpdate",
    j1_hello_comment: "J1_hello_comment",
    j1_if: "J1_if",
    j1_if_then: "J1_if_then",
    j1_if_then_for: "J1_if_then_for",
    j1_ifthenelse: "J1_ifThenElse",
    j1_implements: "J1_implements",
    j1_intarraydecl1: "J1_IntArrayDecl1",
    j1_intarraydecl2: "J1_IntArrayDecl2",
    j1_intcast: "J1_IntCast",
    j1_intcharinit: "J1_IntCharInit",
    j1_integerfun: "J1_integerFun",
    j1_integerfun1: "J1_integerFun1",
    j1_integerfun3: "J1_integerFun3",
    j1_intinit: "J1_IntInit",
    j1_intliterals: "J1_intliterals",
    j1_intminusfoo: "J1_intminusfoo",
    j1_intrange_minnegativeint: "J1_IntRange_MinNegativeInt",
    j1_isthisacast: "J1_IsThisACast",
    j1_lazybooleanoperations: "J1_lazybooleanoperations",
    j1_maxint_comment: "J1_maxint_comment",
    j1_minuschar: "J1_minuschar",
    j1_minusminusminus: "J1_minusminusminus",
    j1_namedtypearray: "J1_NamedTypeArray",
    j1_negativebytecast: "J1_NegativeByteCast",
    j1_negativecharcast: "J1_NegativeCharCast",
    j1_negativeintcast1: "J1_NegativeIntCast1",
    j1_negativeintcast2: "J1_NegativeIntCast2",
    j1_negativeintcast3: "J1_negativeintcast3",
    j1_negativeonebytebytecast: "J1_NegativeOneByteByteCast",
    j1_negativeonebytecharcast: "J1_NegativeOneByteCharCast",
    j1_negativeonebyteintcast: "J1_NegativeOneByteIntCast",
    j1_negativeonebyteshortcast: "J1_NegativeOneByteShortCast",
    j1_negativeshortcast: "J1_NegativeShortCast",
    j1_newobject: "J1_newobject",
    j1_nonemptyconstructor: "J1_nonemptyconstructor",
    j1_nullinstanceof1: "J1_nullinstanceof1",
    j1_nullliteral: "J1_nullliteral",
    j1_octal_escape: "J1_octal_escape",
    j1_octal_escape2: "J1_octal_escape2",
    j1_octal_escape3: "J1_octal_escape3",
    j1_octal_escape4: "J1_octal_escape4",
    j1_octal_escape5: "J1_octal_escape5",
    j1_primitivecasts: "J1_primitivecasts",
    j1_protected: "J1_protected",
    j1_protectedfields: "J1_protectedfields",
    j1_publicclasses: "J1_publicclasses",
    j1_publicconstructors: "J1_publicconstructors",
    j1_publicfields: "J1_publicfields",
    j1_publicmethods: "J1_publicmethods",
    j1_simpletypearray: "J1_SimpleTypeArray",
    j1_smallint: "J1_SmallInt",
    j1_staticmethoddeclaration: "J1_staticmethoddeclaration",
    j1_stringliteralinvoke: "J1_stringliteralinvoke",
    j1_stringliterals: "J1_stringliterals",
    j1_weird_chars: "J1_weird_chars",
    j1w_interface: "J1w_Interface",
    j1w_restrictednative: "J1w_RestrictedNative",
    j1w_staticfield: "J1w_StaticField",
    j2_staticfielddecl: "J2_staticFieldDecl",
    j2_staticfielddeclaration: "J2_staticfielddeclaration",
}
