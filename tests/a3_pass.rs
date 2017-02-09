extern crate juicyj;

macro_rules! a3_pass_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let filename: String = format!("tests/cases/a3/pass/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

macro_rules! a3_pass_tests_folders {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let paths = std::fs::read_dir(format!("tests/cases/a3/pass/{}", $case)).unwrap();
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

a3_pass_tests! {
    j1_5_ambiguousname_fieldvstype: "J1_5_AmbiguousName_FieldVsType",
    j1_5_ambiguousname_fieldvstype_initializer: "J1_5_AmbiguousName_FieldVsType_Initializer",
    j1_5_ambiguousname_localvsfield: "J1_5_AmbiguousName_LocalVsField",
    j1_5_ambiguousname_localvsfield_sameline: "J1_5_AmbiguousName_LocalVsField_SameLine",
    j1_5_ambiguousname_localvstype: "J1_5_AmbiguousName_LocalVsType",
    j1_5_forwardreference_arraylength: "J1_5_ForwardReference_ArrayLength",
    j1_5_forwardreference_equalinfix: "J1_5_ForwardReference_EqualInfix",
    j1_5_forwardreference_explicitthis_inassignment:
        "J1_5_ForwardReference_ExplicitThis_InAssignment",
    j1_5_forwardreference_sameline: "J1_5_ForwardReference_SameLine",
    j1_6_assignable_object_objectarray: "J1_6_Assignable_Object_ObjectArray",
    j1_6_assignmentinarraylength: "J1_6_AssignmentInArrayLength",
    j1_6_assignmentinnotarraylength: "J1_6_AssignmentInNotArrayLength",
    j1_6_staticmethodcall_thisinarg: "J1_6_StaticMethodCall_ThisInArg",
    j1_a_concatinsimpleinvoke: "J1_A_ConcatInSimpleInvoke",
    j1_a_concatinstaticinvoke: "J1_A_ConcatInStaticInvoke",
    j1_array: "J1_array",
    j1_arrayaccess: "J1_arrayAccess",
    j1_arrayaccess_cast: "J1_ArrayAccess_Cast",
    j1_arrayaccess_methodinvocation: "J1_ArrayAccess_MethodInvocation",
    j1_arraycast: "J1_ArrayCast",
    j1_arrayinstanceof1: "J1_arrayinstanceof1",
    j1_arrayinstanceof2: "J1_arrayinstanceof2",
    j1_arrayinterfaces: "J1_ArrayInterfaces",
    j1_arraylength: "J1_arraylength",
    j1_assign_object_to_object: "J1_assign_Object_to_Object",
    j1_backwardref: "J1_backwardRef",
    j1_backwardsfieldref: "J1_backwardsFieldRef",
    j1_bigbyteinit: "J1_BigByteInit",
    j1_bigcharcharinit: "J1_BigCharCharInit",
    j1_bigshortfrombyteinit: "J1_BigShortFromByteInit",
    j1_bigshortinit: "J1_BigShortInit",
    j1_boolean: "J1_boolean",
    j1_booleanliterals: "J1_booleanliterals",
    j1_byte: "J1_byte",
    j1_bytecast: "J1_ByteCast",
    j1_bytecharinit2: "J1_ByteCharInit2",
    j1_byteinit: "J1_ByteInit",
    j1_callstaticmethods: "J1_callstaticmethods",
    j1_cast_to_same_type: "J1_cast_to_same_type",
    j1_castarrayaccess: "J1_castarrayaccess",
    j1_castmultiple1: "J1_castMultiple1",
    j1_castmultiple2: "J1_castMultiple2",
    j1_castmultiple: "J1_castMultiple",
    j1_castprimarymethodinvocation: "J1_castprimarymethodinvocation",
    j1_castthis: "J1_castthis",
    j1_charcast: "J1_CharCast",
    j1_charcharinit1: "J1_CharCharInit1",
    j1_closestmatchconstructor1: "J1_closestMatchConstructor1",
    j1_closestmatchmultiplepath1: "J1_ClosestMatchMultiplePath1",
    j1_closestmatchmultiplepath2: "J1_ClosestMatchMultiplePath2",
    j1_closestmethod2: "J1_ClosestMethod2",
    j1_closestmethod3: "J1_ClosestMethod3",
    j1_closestmethod4: "J1_ClosestMethod4",
    j1_constructoroverloading: "J1_constructoroverloading",
    j1_evalmethodinvocationfromparexp: "J1_evalMethodInvocationFromParExp",
    j1_fieldinit2: "J1_fieldinit2",
    j1_fieldinit: "J1_fieldinit",
    j1_fieldinit_forward_ref2: "J1_fieldinit_forward_ref2",
    j1_fieldinit_forward_ref: "J1_fieldinit_forward_ref",
    j1_fieldinowninit: "J1_fieldInOwnInit",
    j1_fieldowninit1: "J1_fieldOwnInit1",
    j1_fieldowninit2: "J1_fieldOwnInit2",
    j1_fieldrestrictionduringinit: "J1_FieldRestrictionDuringInit",
    j1_formal_with_same_name_as_field: "J1_formal_with_same_name_as_field",
    j1_formalindex: "J1_formalindex",
    j1_forwardfield1: "J1_forwardfield1",
    j1_forwardfield2: "J1_forwardfield2",
    j1_good_dot: "J1_good_dot",
    j1_implicitstringconcatenation: "J1_implicitstringconcatenation",
    j1_implicitthisforfields: "J1_implicitthisforfields",
    j1_implicitthisformethods: "J1_implicitthisformethods",
    j1_instanceof: "J1_instanceof",
    j1_instanceof_array2: "J1_instanceof_array2",
    j1_instanceof_array: "J1_instanceof_array",
    j1_instanceof_string: "J1_instanceof_string",
    j1_int: "J1_int",
    j1_intstringadd: "J1_intstringadd",
    j1_length_field_not_array: "J1_length_field_not_array",
    j1_localdeclaccess: "J1_localDeclAccess",
    j1_methodinvocationqualified: "J1_methodInvocationQualified",
    j1_methodoverloading: "J1_methodoverloading",
    j1_methodwitharglist: "J1_methodWithArgList",
    j1_namelinking3: "J1_namelinking3",
    j1_nestedblocks: "J1_nestedblocks",
    j1_nestedcast: "J1_nestedcast",
    j1_nonthisfieldaccess: "J1_nonthisfieldaccess",
    j1_nullinstanceof1: "J1_nullinstanceof1",
    j1_onebytebytecast: "J1_OneByteByteCast",
    j1_onebytecharcast: "J1_OneByteCharCast",
    j1_onebyteintcast: "J1_OneByteIntCast",
    j1_onebyteshortcast: "J1_OneByteShortCast",
    j1_primitivecasts: "J1_primitivecasts",
    j1_referencecasts: "J1_referencecasts",
    j1_samestaticinvoketwice: "J1_samestaticinvoketwice",
    j1_short: "J1_short",
    j1_shortcast: "J1_ShortCast",
    j1_shortcharinit2: "J1_ShortCharInit2",
    j1_shortfrombyteinit: "J1_ShortFromByteInit",
    j1_shortinit: "J1_ShortInit",
    j1_sideeffects_obj3: "J1_sideeffects_obj3",
    j1_staticfield_accessfromclass: "J1_StaticField_AccessFromClass",
    j1_staticmethodinvocation: "J1_staticMethodInvocation",
    j1_typecheck_array: "J1_typecheck_array",
    j1_typecheck_constructor_invocation: "J1_typecheck_constructor_invocation",
    j1_typecheck_equality: "J1_typecheck_equality",
    j1_typecheck_expstm: "J1_typecheck_expstm",
    j1_typecheck_if1: "J1_typecheck_if1",
    j1_typecheck_if2: "J1_typecheck_if2",
    j1_typecheck_instanceof1: "J1_typecheck_instanceof1",
    j1_typecheck_instanceof2: "J1_typecheck_instanceof2",
    j1_typecheck_instanceof3: "J1_typecheck_instanceof3",
    j1_typecheck_instanceof4: "J1_typecheck_instanceof4",
    j1_typecheck_instanceof5: "J1_typecheck_instanceof5",
    j1_typecheck_instanceof6: "J1_typecheck_instanceof6",
    j1_typecheck_instanceof7: "J1_typecheck_instanceof7",
    j1_typecheck_instanceof: "J1_typecheck_instanceof",
    j1_typecheck_plus: "J1_typecheck_plus",
    j1_typecheck_return: "J1_typecheck_return",
    j1_typecheck_static_invocation1: "J1_typecheck_static_invocation1",
    j1_typecheck_while: "J1_typecheck_while",
    j1_wrapper_classes_eq: "J1_wrapper_classes_eq",
    j2_5_forwardreference_staticfield: "J2_5_ForwardReference_StaticField",
    j2_backwardsstaticfieldref: "J2_backwardsStaticFieldRef",
    j2_exactmatchconstructor3: "J2_exactMatchConstructor3",
    j2_fieldinit_forward_ref: "J2_fieldinit_forward_ref",
    j2_forwardref: "J2_forwardRef",
    j2_static_decl: "J2_static_decl",
    j2_static_shared: "J2_static_shared",
    j2_staticfield2: "J2_staticField2",
    j2_staticfield: "J2_staticField",
    j2_staticfielddecl: "J2_staticFieldDecl",
}

a3_pass_tests_folders! {
    j1_5_ambiguousname_defaultpackagenotvisible: "J1_5_AmbiguousName_DefaultPackageNotVisible",
    j1_6_protectedaccess_implicitsuper: "J1_6_ProtectedAccess_ImplicitSuper",
    j1_6_protectedaccess_instancefield_subvar: "J1_6_ProtectedAccess_InstanceField_SubVar",
    j1_6_protectedaccess_instancefield_this: "J1_6_ProtectedAccess_InstanceField_This",
    j1_6_protectedaccess_instancefield_thisvar: "J1_6_ProtectedAccess_InstanceField_ThisVar",
    j1_6_protectedaccess_instancemethod_subvar: "J1_6_ProtectedAccess_InstanceMethod_SubVar",
    j1_6_protectedaccess_instancemethod_this: "J1_6_ProtectedAccess_InstanceMethod_This",
    j1_6_protectedaccess_instancemethod_thisvar: "J1_6_ProtectedAccess_InstanceMethod_ThisVar",
    j1_6_protectedaccess_staticmethod_sub: "J1_6_ProtectedAccess_StaticMethod_Sub",
    j1_6_protectedaccess_staticmethod_super: "J1_6_ProtectedAccess_StaticMethod_Super",
    j1_6_protectedaccess_staticmethod_this: "J1_6_ProtectedAccess_StaticMethod_This",
    j1_accessstaticfield: "J1_accessstaticfield",
    j1_ambiguousinvoke: "J1_ambiguousInvoke",
    j1_arraycast1: "J1_ArrayCast1",
    j1_arraycast2: "J1_ArrayCast2",
    j1_arraycast3: "J1_ArrayCast3",
    j1_arraycast4: "J1_ArrayCast4",
    j1_evalmethodinvocationfromarray: "J1_evalMethodInvocationFromArray",
    j1_evalmethodinvocationfromlit: "J1_evalMethodInvocationFromLit",
    j1_evalmethodinvocationfrommethod: "J1_evalMethodInvocationFromMethod",
    j1_evalmethodinvocationfromobject: "J1_evalMethodInvocationFromObject",
    j1_evalmethodinvocationfromthis: "J1_evalMethodInvocationFromThis",
    j1_interface_null: "J1_interface_null",
    j1_interfaceassignable: "J1_interfaceassignable",
    j1_interfaceobject: "J1_InterfaceObject",
    j1_namedcast2: "J1_NamedCast2",
    j1_namedcast3: "J1_NamedCast3",
    j1_namedcast4: "J1_NamedCast4",
    j1_protectedaccess1: "J1_ProtectedAccess1",
    j1_protectedaccess2: "J1_ProtectedAccess2",
    j1_protectedaccess4: "J1_ProtectedAccess4",
    j1_supermethod_override11: "J1_supermethod_override11",
    j1_typecheck_assignment: "J1_typecheck_assignment",
    j2_6_ambiguousname_staticfieldaccess: "J2_6_AmbiguousName_StaticFieldAccess",
    j2_6_protectedaccess_staticfield_sub: "J2_6_ProtectedAccess_StaticField_Sub",
    j2_6_protectedaccess_staticfield_super: "J2_6_ProtectedAccess_StaticField_Super",
    j2_6_protectedaccess_staticfield_this: "J2_6_ProtectedAccess_StaticField_This",
    j2_implicitstaticmethod: "J2_implicitStaticMethod",
    j2_interfaces: "J2_interfaces",
    j2_protectedaccess3: "J2_ProtectedAccess3",
}
