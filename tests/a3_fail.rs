extern crate juicyj;
extern crate walkdir;

macro_rules! a3_fail_tests {
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

            let filename: String = format!("tests/cases/a3/fail/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);
            asts.push(juicyj::scanner::tests::scan_or_assert(&filename, &src));

            juicyj::analysis::tests::analyze_and_assert(&asts);
        }
    )*
    }
}

macro_rules! a3_fail_tests_folders {
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

            for path in walkdir::WalkDir::new(format!("tests/cases/a3/fail/{}", $case)) {
                match path.unwrap().path().to_str() {
                    Some(filename) if filename.ends_with(".java") => {
                        let src: String = juicyj::scanner::read_src_file(&filename.to_string());
                        if format!("{}", $case).starts_with("Je_3_SingleTypeImport_ClashWithEach") {
                            // these fail as AST building
                            juicyj::scanner::tests::scan_and_assert(&filename, &src);
                        } else {
                            asts.push(juicyj::scanner::tests::scan_or_assert(&filename, &src));
                        }
                    }
                    _ => (),
                }
            }

            if !format!("{}", $case).starts_with("Je_3_SingleTypeImport_ClashWithEach") {
                juicyj::analysis::tests::analyze_and_assert(&asts);
            }
        }
    )*
    }
}

a3_fail_tests! {
    je_16_closestmatch_array: "Je_16_ClosestMatch_Array",
    je_16_closestmatch_constructor_noclosestmatch_this:
        "Je_16_ClosestMatch_Constructor_NoClosestMatch_This",
    je_16_incdec_final_arraylengthdec: "Je_16_IncDec_Final_ArrayLengthDec",
    je_16_incdec_final_arraylengthinc: "Je_16_IncDec_Final_ArrayLengthInc",
    je_16_incdec_final_postdec: "Je_16_IncDec_Final_PostDec",
    je_16_incdec_final_postinc: "Je_16_IncDec_Final_PostInc",
    je_16_incdec_final_predec: "Je_16_IncDec_Final_PreDec",
    je_16_incdec_final_preinc: "Je_16_IncDec_Final_PreInc",
    je_16_incdec_stringpostdec: "Je_16_IncDec_StringPostDec",
    je_16_incdec_stringpostinc: "Je_16_IncDec_StringPostInc",
    je_16_incdec_stringpredec: "Je_16_IncDec_StringPreDec",
    je_16_incdec_stringpreinc: "Je_16_IncDec_StringPreInc",
    je_16_methodpresent_wrongname_array: "Je_16_MethodPresent_WrongName_Array",
    je_16_multiarraycreation_assign_1: "Je_16_MultiArrayCreation_Assign_1",
    je_16_multiarraycreation_null: "Je_16_MultiArrayCreation_Null",
    je_16_staticthis_staticfieldinitializer: "Je_16_StaticThis_StaticFieldInitializer",
    je_16_superthis_invalidsuperparameter: "Je_16_SuperThis_InvalidSuperParameter",
    je_16_superthis_invalidthisparameter: "Je_16_SuperThis_InvalidThisParameter",
    je_1_cast_namedcastnegativeint: "Je_1_Cast_NamedCastNegativeint",
    je_1_complement_ofintliteral: "Je_1_Complement_OfIntLiteral",
    je_1_dot_parenthesizedtype_field: "Je_1_Dot_ParenthesizedType_Field",
    je_1_dot_parenthesizedtype_method: "Je_1_Dot_ParenthesizedType_Method",
    je_1_instanceof_primitive: "Je_1_InstanceOf_Primitive",
    je_1_methodinvocation_primitive: "Je_1_MethodInvocation_Primitive",
    je_2_cast_negativecomplexexpressiontonamedtype:
        "Je_2_Cast_NegativeComplexExpressionToNamedType",
    je_2_cast_negativetonamedtype: "Je_2_Cast_NegativeToNamedType",
    je_2_cast_negativetoqualifiednamedtype: "Je_2_Cast_NegativeToQualifiedNamedType",
    je_2_for_localvarinupdate: "Je_2_For_LocalVarInUpdate",
    je_5_ambiguousinvoke_localinowninitializer: "Je_5_AmbiguousInvoke_LocalInOwnInitializer",
    je_5_ambiguousinvoke_static_typenonexisting: "Je_5_AmbiguousInvoke_Static_TypeNonExisting",
    je_5_ambiguousname_fieldvstype_initializer: "Je_5_AmbiguousName_FieldVsType_Initializer",
    je_5_ambiguousname_local_usebeforedeclare: "Je_5_AmbiguousName_Local_UseBeforeDeclare",
    je_5_ambiguousname_nodeclaration: "Je_5_AmbiguousName_NoDeclaration",
    je_5_ambiguousname_samepackageandclassname: "Je_5_AmbiguousName_SamePackageAndClassName",
    je_5_forwardreference_arraylength: "Je_5_ForwardReference_ArrayLength",
    je_5_forwardreference_fielddeclaredlater: "Je_5_ForwardReference_FieldDeclaredLater",
    je_5_forwardreference_fielddeclaredlater_complexexp:
        "Je_5_ForwardReference_FieldDeclaredLater_ComplexExp",
    je_5_forwardreference_fieldinowninitializer_complexexpression:
        "Je_5_ForwardReference_FieldInOwnInitializer_ComplexExpression",
    je_5_forwardreference_fieldinowninitializer_direct:
        "Je_5_ForwardReference_FieldInOwnInitializer_Direct",
    je_5_forwardreference_fieldinowninitializer_readafterassignment:
        "Je_5_ForwardReference_FieldInOwnInitializer_ReadAfterAssignment",
    je_5_forwardreference_fieldinowninitializer_rightsideofassignment:
        "Je_5_ForwardReference_FieldInOwnInitializer_RightSideOfAssignment",
    je_5_forwardreference_inassignment: "Je_5_ForwardReference_InAssignment",
    je_5_forwardreference_methodcall: "Je_5_ForwardReference_MethodCall",
    je_6_array_nonnumericindex: "Je_6_Array_NonNumericIndex",
    je_6_array_nulltypeindex: "Je_6_Array_NullTypeIndex",
    je_6_arraylength_invoke: "Je_6_ArrayLength_Invoke",
    je_6_assignable_array_object: "Je_6_Assignable_Array_Object",
    je_6_assignable_byte_char: "Je_6_Assignable_byte_char",
    je_6_assignable_byte_int: "Je_6_Assignable_byte_int",
    je_6_assignable_bytearray_intarray: "Je_6_Assignable_byteArray_intArray",
    je_6_assignable_cast_intarray_int: "Je_6_Assignable_Cast_intArray_int",
    je_6_assignable_char_byte_1: "Je_6_Assignable_char_byte_1",
    je_6_assignable_char_byte_2: "Je_6_Assignable_char_byte_2",
    je_6_assignable_char_int: "Je_6_Assignable_char_int",
    je_6_assignable_condition: "Je_6_Assignable_Condition",
    je_6_assignable_condition_simpletype: "Je_6_Assignable_Condition_SimpleType",
    je_6_assignable_condition_while: "Je_6_Assignable_Condition_While",
    je_6_assignable_instanceof_result: "Je_6_Assignable_Instanceof_Result",
    je_6_assignable_instanceof_simpletype: "Je_6_Assignable_Instanceof_SimpleType",
    je_6_assignable_instanceof_simpletypeofsimpletype:
        "Je_6_Assignable_Instanceof_SimpleTypeOfSimpleType",
    je_6_assignable_int_intarray: "Je_6_Assignable_int_intArray",
    je_6_assignable_int_null: "Je_6_Assignable_int_null",
    je_6_assignable_intarray_bytearray: "Je_6_Assignable_intArray_byteArray",
    je_6_assignable_intarray_int: "Je_6_Assignable_intArray_int",
    je_6_assignable_namedcastofcomplement: "Je_6_Assignable_NamedCastOfComplement",
    je_6_assignable_nonstaticfield: "Je_6_Assignable_NonstaticField",
    je_6_assignable_reftype_reftypearray: "Je_6_Assignable_RefType_RefTypeArray",
    je_6_assignable_resulttypeofassignment: "Je_6_Assignable_ResultTypeOfAssignment",
    je_6_assignable_return_tosubtype: "Je_6_Assignable_Return_ToSubType",
    je_6_assignable_return_void: "Je_6_Assignable_Return_Void",
    je_6_assignable_return_voidinvoidmethod: "Je_6_Assignable_Return_VoidInVoidMethod",
    je_6_assignable_returninelse: "Je_6_Assignable_ReturnInElse",
    je_6_assignable_short_char: "Je_6_Assignable_short_char",
    je_6_assignable_short_int: "Je_6_Assignable_short_int",
    je_6_assignable_tosubtype_fieldinit: "Je_6_Assignable_ToSubtype_FieldInit",
    je_6_assignable_valuereturn_inconstructor: "Je_6_Assignable_ValueReturn_InConstructor",
    je_6_binopexp_logicalbitwise: "Je_6_BinopExp_LogicalBitwise",
    je_6_closestmatch_arraytypes: "Je_6_ClosestMatch_ArrayTypes",
    je_6_closestmatch_constructor_noclosestmatch_simpletypes:
        "Je_6_ClosestMatch_Constructor_NoClosestMatch_SimpleTypes",
    je_6_closestmatch_multipleclosest_1: "Je_6_ClosestMatch_MultipleClosest_1",
    je_6_closestmatch_multipleclosest_2: "Je_6_ClosestMatch_MultipleClosest_2",
    je_6_closestmatch_multipleclosest_3: "Je_6_ClosestMatch_MultipleClosest_3",
    je_6_closestmatch_multipleclosest_simpletypes: "Je_6_ClosestMatch_MultipleClosest_SimpleTypes",
    je_6_constructor_wrongname: "Je_6_Constructor_WrongName",
    je_6_constructorpresent_argumenttypemismatch: "Je_6_ConstructorPresent_ArgumentTypeMismatch",
    je_6_constructorpresent_illegalconversion: "Je_6_ConstructorPresent_IllegalConversion",
    je_6_constructorpresent_multipleargumentsonemismatch:
        "Je_6_ConstructorPresent_MultipleArgumentsOneMismatch",
    je_6_constructorpresent_samelastarg: "Je_6_ConstructorPresent_SameLastArg",
    je_6_constructorpresent_toofewarguments: "Je_6_ConstructorPresent_TooFewArguments",
    je_6_constructorpresent_toomanyarguments: "Je_6_ConstructorPresent_TooManyArguments",
    je_6_equality_int: "Je_6_Equality_int",
    je_6_equality_int_namedtype: "Je_6_Equality_int_NamedType",
    je_6_equality_stringinteger: "Je_6_Equality_StringInteger",
    je_6_equality_void: "Je_6_Equality_Void",
    je_6_expression_stringconcat_void: "Je_6_Expression_StringConcat_Void",
    je_6_finalfield_arraylength: "Je_6_FinalField_ArrayLength",
    je_6_for_nullincondition: "Je_6_For_NullInCondition",
    je_6_instanceof_primitive_1: "Je_6_InstanceOf_Primitive_1",
    je_6_instanceof_primitive_2: "Je_6_InstanceOf_Primitive_2",
    je_6_instanceof_primitive_3: "Je_6_InstanceOf_Primitive_3",
    je_6_instantiateabstract: "Je_6_InstantiateAbstract",
    je_6_instantiateinterface: "Je_6_InstantiateInterface",
    je_6_methodinvocation_nonjoos_returntype: "Je_6_MethodInvocation_NonJoos_ReturnType",
    je_6_methodpresent_argumenttypemismatch: "Je_6_MethodPresent_ArgumentTypeMismatch",
    je_6_methodpresent_illegalconversion: "Je_6_MethodPresent_IllegalConversion",
    je_6_methodpresent_multipleargumentsonemismatch:
        "Je_6_MethodPresent_MultipleArgumentsOneMismatch",
    je_6_methodpresent_nonstatic_samelastarg: "Je_6_MethodPresent_Nonstatic_SameLastArg",
    je_6_methodpresent_static_samelastarg: "Je_6_MethodPresent_Static_SameLastArg",
    je_6_methodpresent_toofewarguments: "Je_6_MethodPresent_TooFewArguments",
    je_6_methodpresent_toomanyarguments: "Je_6_MethodPresent_TooManyArguments",
    je_6_nonstaticaccesstostatic_field: "Je_6_NonStaticAccessToStatic_Field",
    je_6_nonstaticaccesstostatic_method: "Je_6_NonStaticAccessToStatic_Method",
    je_6_staticaccesstonontatic_field: "Je_6_StaticAccessToNontatic_Field",
    je_6_staticaccesstonontatic_method: "Je_6_StaticAccessToNontatic_Method",
    je_6_staticthis_afterstaticinvoke: "Je_6_StaticThis_AfterStaticInvoke",
    je_6_staticthis_invokenonstatic: "Je_6_StaticThis_InvokeNonStatic",
    je_6_staticthis_invokenonstatic_implicit: "Je_6_StaticThis_InvokeNonstatic_Implicit",
    je_6_staticthis_invokestatic: "Je_6_StaticThis_InvokeStatic",
    je_6_staticthis_nonstaticfield: "Je_6_StaticThis_NonstaticField",
    je_6_staticthis_nonstaticfield_implicitthis: "Je_6_StaticThis_NonStaticField_ImplicitThis",
    je_6_stringminus: "Je_6_StringMinus",
    je_badconstructorname: "Je_BadConstructorName",
}

a3_fail_tests_folders! {
    je_16_protectedaccess_staticfield_sub_declaredinsub:
        "Je_16_ProtectedAccess_StaticField_Sub_DeclaredInSub",
    je_3_resolve_linktocorrectpackage: "Je_3_Resolve_LinkToCorrectPackage",
    je_5_ambiguousname_defaultpackagenotvisible: "Je_5_AmbiguousName_DefaultPackageNotVisible",
    je_5_ambiguousname_linktofirstfound: "Je_5_AmbiguousName_LinkToFirstFound",
    je_6_assignable_tosubtype: "Je_6_Assignable_ToSubtype",
    je_6_assignable_tosubtype_declinit: "Je_6_Assignable_ToSubtype_DeclInit",
    je_6_closestmatch_constructor_noclosestmatch: "Je_6_ClosestMatch_Constructor_NoClosestMatch",
    je_6_constructorpresent_presentinsubclass: "Je_6_ConstructorPresent_PresentInSubclass",
    je_6_constructorpresent_super_nodefault: "Je_6_ConstructorPresent_Super_NoDefault",
    je_6_methodpresent_presentinsubclass: "Je_6_MethodPresent_PresentInSubclass",
    je_6_protectedaccess_classcreation_sub: "Je_6_ProtectedAccess_ClassCreation_Sub",
    je_6_protectedaccess_classcreation_super: "Je_6_ProtectedAccess_ClassCreation_Super",
    je_6_protectedaccess_constructor: "Je_6_ProtectedAccess_Constructor",
    je_6_protectedaccess_external: "Je_6_ProtectedAccess_External",
    je_6_protectedaccess_instancefield_norelation_external:
        "Je_6_ProtectedAccess_InstanceField_NoRelation_External",
    je_6_protectedaccess_instancefield_norelation_internal:
        "Je_6_ProtectedAccess_InstanceField_NoRelation_Internal",
    je_6_protectedaccess_instancefield_subdeclare_subvar:
        "Je_6_ProtectedAccess_InstanceField_SubDeclare_SubVar",
    je_6_protectedaccess_instancefield_supervar: "Je_6_ProtectedAccess_InstanceField_SuperVar",
    je_6_protectedaccess_instancemethod_subdeclare_subvar:
        "Je_6_ProtectedAccess_InstanceMethod_SubDeclare_SubVar",
    je_6_protectedaccess_instancemethod_supervar: "Je_6_ProtectedAccess_InstanceMethod_SuperVar",
    je_6_protectedaccess_method_outsidepackage_notbysubclass:
        "Je_6_ProtectedAccess_Method_OutsidePackage_NotBySubclass",
    je_6_protectedaccess_method_outsidepackage_notinsubclass:
        "Je_6_ProtectedAccess_Method_OutsidePackage_NotInSubclass",
    je_6_protectedaccess_readfield_outsidepackage_notbysubclass:
        "Je_6_ProtectedAccess_ReadField_OutsidePackage_NotBySubclass",
    je_6_protectedaccess_readfield_outsidepackage_notinsubclass:
        "Je_6_ProtectedAccess_ReadField_OutsidePackage_NotInSubclass",
    je_6_protectedaccess_staticmethod_sub_declaredinsub:
        "Je_6_ProtectedAccess_StaticMethod_Sub_DeclaredInSub",
    je_6_protectedaccess_superconstructor_newexp: "Je_6_ProtectedAccess_SuperConstructor_NewExp",
    je_6_protectedaccess_twosubtypes: "Je_6_ProtectedAccess_TwoSubtypes",
    je_6_protectedaccess_writefield_outsidepackage_notbysubclass:
        "Je_6_ProtectedAccess_WriteField_OutsidePackage_NotBySubclass",
    je_6_protectedaccess_writefield_outsidepackage_notinsubclass:
        "Je_6_ProtectedAccess_WriteField_OutsidePackage_NotInSubclass",
    je_accesstostaticfieldwithimplicitthis: "Je_AccessToStaticFieldWithImplicitThis",
}
