extern crate juicyj;

macro_rules! a5_pass_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let filename: String = format!("tests/cases/a5/pass/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

macro_rules! a5_pass_tests_folders {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let paths = std::fs::read_dir(format!("tests/cases/a5/pass/{}", $case)).unwrap();
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

a5_pass_tests! {
    j1_01: "J1_01",
    j1_1_instanceof_inlazyexp: "J1_1_Instanceof_InLazyExp",
    j1_1_instanceof_ofadditiveexpression: "J1_1_Instanceof_OfAdditiveExpression",
    j1_1_instanceof_ofcastexpression: "J1_1_Instanceof_OfCastExpression",
    j1_300locals: "J1_300locals",
    j1_6_assignable_object_objectarray: "J1_6_Assignable_Object_ObjectArray",
    j1_6_assignmentinarraylength: "J1_6_AssignmentInArrayLength",
    j1_a_addressnotequal: "J1_A_AddressNotEqual",
    j1_a_arraybaseinassignment: "J1_A_ArrayBaseInAssignment",
    j1_a_arraystoreload: "J1_A_ArrayStoreLoad",
    j1_a_assignmentinlazyor: "J1_A_AssignmentInLazyOr",
    j1_a_booleanarray_external: "J1_A_BooleanArray_External",
    j1_a_clonewithargs: "J1_A_CloneWithArgs",
    j1_a_complement_sideeffect: "J1_A_Complement_SideEffect",
    j1_a_concatinsimpleinvoke: "J1_A_ConcatInSimpleInvoke",
    j1_a_concatinstaticinvoke: "J1_A_ConcatInStaticInvoke",
    j1_a_conditionals_noinstructionafterifelse: "J1_A_Conditionals_NoInstructionAfterIfElse",
    j1_a_fieldinitialization_before: "J1_A_FieldInitialization_Before",
    j1_a_fieldinitialization_nonconstant_before: "J1_A_FieldInitialization_NonConstant_Before",
    j1_a_greaterorequal: "J1_A_GreaterOrEqual",
    j1_a_lazyeagerandor: "J1_A_LazyEagerAndOr",
    j1_a_lazyeval: "J1_A_LazyEval",
    j1_a_string_byteshortcharint: "J1_A_String_ByteShortCharInt",
    j1_a_stringconstaeq_ane: "J1_A_StringConstAEQ_ANE",
    j1_arithmeticoperations: "J1_arithmeticoperations",
    j1_array: "J1_array",
    j1_arrayaccess: "J1_arrayAccess",
    j1_arraycreateandindex: "J1_ArrayCreateAndIndex",
    j1_arrayinstanceof1: "J1_arrayinstanceof1",
    j1_arrayinstanceof2: "J1_arrayinstanceof2",
    j1_backwardref: "J1_backwardRef",
    j1_bigbyteinit: "J1_BigByteInit",
    j1_bigcharcharinit: "J1_BigCharCharInit",
    j1_bigshortfrombyteinit: "J1_BigShortFromByteInit",
    j1_bigshortinit: "J1_BigShortInit",
    j1_charadd: "J1_charadd",
    j1_concat_in_binop: "J1_concat_in_binop",
    j1_concatinmethods: "J1_concatInMethods",
    j1_constructorbodycast: "J1_constructorbodycast",
    j1_divdiv: "J1_divdiv",
    j1_fieldinit: "J1_fieldinit",
    j1_fieldinit_forward_ref: "J1_fieldinit_forward_ref",
    j1_fieldinit_forward_ref2: "J1_fieldinit_forward_ref2",
    j1_hello: "J1_Hello",
    j1_implicitstringconcatenation: "J1_implicitstringconcatenation",
    j1_instanceof_array: "J1_instanceof_array",
    j1_instanceof_array2: "J1_instanceof_array2",
    j1_intstringadd: "J1_intstringadd",
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
    j1_nestedcast: "J1_nestedcast",
    j1_random_arithmetic: "J1_random_arithmetic",
    j1_random_arithmetic_var: "J1_random_arithmetic_var",
    j1_sideeffect1: "J1_sideeffect1",
    j1_sideeffect2: "J1_sideeffect2",
    j1_sideeffect3: "J1_sideeffect3",
    j1_sideeffect4: "J1_sideeffect4",
    j1_sideeffect5: "J1_sideeffect5",
    j1_sideeffect6: "J1_sideeffect6",
    j1_sideeffect7: "J1_sideeffect7",
    j1_sideeffect8: "J1_sideeffect8",
    j1_sideeffects_array: "J1_sideeffects_array",
    j1_sideeffects_array2: "J1_sideeffects_array2",
    j1_sideeffects_array3: "J1_sideeffects_array3",
    j1_sideeffects_array4: "J1_sideeffects_array4",
    j1_sideeffects_obj2: "J1_sideeffects_obj2",
    j1_sideeffects_obj3: "J1_sideeffects_obj3",
    j1_sim_and: "J1_sim_and",
    j1_sim_or: "J1_sim_or",
    j1_sim_xor: "J1_sim_xor",
    j1_simpletypearray: "J1_SimpleTypeArray",
    j1_smallint: "J1_SmallInt",
    j1_staticfield_accessfromclass: "J1_StaticField_AccessFromClass",
    j1_staticmethodinvocation: "J1_staticMethodInvocation",
    j1_stringadd: "J1_stringadd",
    j1_stringcast: "J1_StringCast",
    j1_stringconcat: "J1_stringconcat",
    j1_toomuchinc: "J1_toomuchinc",
    j1_typecheck_array: "J1_typecheck_array",
    j1_typecheck_expstm: "J1_typecheck_expstm",
    j1_typecheck_plus: "J1_typecheck_plus",
    j1_while1: "J1_while1",
    j1_while2: "J1_while2",
    j1_whiletrue1: "J1_whiletrue1",
    j1_wildconcat: "J1_WildConcat",
    j1e_a_casttoarray: "J1e_A_CastToArray",
    j1e_a_casttostring: "J1e_A_CastToString",
    j1e_divisionbyzero: "J1e_divisionbyzero",
    j2_a_fieldinitialization_static_before: "J2_A_FieldInitialization_Static_Before",
    j2_a_fieldinitialization_static_nonconstant_before:
        "J2_A_FieldInitialization_Static_NonConstant_Before",
    j2_fieldinit_forward_ref: "J2_fieldinit_forward_ref",
    j2_forwardref: "J2_forwardRef",
}

a5_pass_tests_folders! {
    j1_a_cloneoninterface: "J1_A_CloneOnInterface",
    j1_callbeforereturn: "J1_callbeforereturn",
    j1_sideeffects_obj: "J1_sideeffects_obj",
    j1e_a_castnewexp: "J1e_A_CastNewExp",
}
