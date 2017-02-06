extern crate juicyj;

mod common;

use common::read_src_file;

macro_rules! public_fail_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("tests/cases/a1/fail/{}.java", $case);
            let src: String = read_src_file(&filename);

            let lexer = juicyj::lexer::Lexer::new(&filename, &src);
            for token in lexer.clone().collect::<Vec<Result<_, _>>>() {
                match token {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Lexer Error");
                        assert!(true);
                        return;
                    },
                }
            }

            let mut parser = juicyj::parser::Parser::new(lexer);
            let parse_tree = match parser.get_tree() {
                Ok(pt) => pt,
                Err(_) => {
                    println!("Parser Error");
                    assert!(true);
                    return;
                }
            };

            let mut weeder = juicyj::weeder::Weeder::new(&filename, &parse_tree);
            match weeder.verify(None) {
                Ok(_) => (),
                Err(_) => {
                    println!("Weeder Verification Error");
                    assert!(true);
                    return;
                }
            }

            match juicyj::common::AST::new(&parse_tree) {
                Ok(_) => {
                    println!("No Error Found");
                    assert!(false);
                },
                Err(_) => assert!(true),
            };
        }
    )*
    }
}

public_fail_tests! {
    je_16_circularity_1: "Je_16_Circularity_1",
    je_16_circularity_2: "Je_16_Circularity_2",
    je_16_circularity_3: "Je_16_Circularity_3",
    je_16_circularity_4_rhoshaped: "Je_16_Circularity_4_Rhoshaped",
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
    je_16_multiarraycreation_assign_1: "Je_16_MultiArrayCreation_Assign_1",
    je_16_multiarraycreation_null: "Je_16_MultiArrayCreation_Null",
    je_16_staticthis_argumenttosuper: "Je_16_StaticThis_ArgumentToSuper",
    je_16_staticthis_argumenttothis: "Je_16_StaticThis_ArgumentToThis",
    je_16_superthis_invalidsuperparameter: "Je_16_SuperThis_InvalidSuperParameter",
    je_16_superthis_invalidthisparameter: "Je_16_SuperThis_InvalidThisParameter",
    je_16_throw_nothrows: "Je_16_Throw_NoThrows",
    je_16_throw_notsubclass: "Je_16_Throw_NotSubclass",
    je_16_throw_simpletype: "Je_16_Throw_SimpleType",
    je_16_throw_throwsnotsuperclass: "Je_16_Throw_ThrowsNotSuperclass",
    je_16_throws_this: "Je_16_Throws_This",
    je_17_unreachable_afterthrow: "Je_17_Unreachable_AfterThrow",
    je_17_unreachable_afterthrowinconditional: "Je_17_Unreachable_AfterThrowInConditional",
    je_1_abstractclass_abstractconstructor: "Je_1_AbstractClass_AbstractConstructor",
    je_1_abstractclass_final: "Je_1_AbstractClass_Final",
    je_1_abstractmethod_body: "Je_1_AbstractMethod_Body",
    je_1_abstractmethod_emptybody: "Je_1_AbstractMethod_EmptyBody",
    je_1_abstractmethod_final: "Je_1_AbstractMethod_Final",
    je_1_abstractmethod_static: "Je_1_AbstractMethod_Static",
    je_1_abstractmethodcannotbefinal: "Je_1_AbstractMethodCannotBeFinal",
    je_1_access_privatelocal: "Je_1_Access_PrivateLocal",
    je_1_access_protectedlocal: "Je_1_Access_ProtectedLocal",
    je_1_access_publiclocal: "Je_1_Access_PublicLocal",
    je_1_array_data: "Je_1_Array_Data",
    je_1_array_data_empty: "Je_1_Array_Data_Empty",
    je_1_array_onvariablenameindecl: "Je_1_Array_OnVariableNameInDecl",
    je_1_cast_doubleparenthese: "Je_1_Cast_DoubleParenthese",
    je_1_cast_expression: "Je_1_Cast_Expression",
    je_1_cast_lefthandsideofassignment_1: "Je_1_Cast_LeftHandSideOfAssignment_1",
    je_1_cast_lefthandsideofassignment_2: "Je_1_Cast_LeftHandSideOfAssignment_2",
    je_1_cast_nonstaticfield: "Je_1_Cast_NonstaticField",
    je_1_cast_noparenthesis: "Je_1_Cast_NoParenthesis",
    je_1_cast_tomethodinvoke: "Je_1_Cast_ToMethodInvoke",
    je_1_casttoarraylvalue: "Je_1_CastToArrayLvalue",
    je_1_classdeclaration_wrongfilename: "Je_1_ClassDeclaration_WrongFileName",
    je_1_classdeclaration_wrongfilename_dotfoo: "Je_1_ClassDeclaration_WrongFileName_Dot.foo",
    je_1_classdeclaration_wrongfilename_suffix: "Je_1_ClassDeclaration_WrongFileName_Suffix",
    je_1_classinstantiation_instantiatesimpletype: "Je_1_ClassInstantiation_InstantiateSimpleType",
    je_1_classinstantiation_instantiatesimplevalue:
        "Je_1_ClassInstantiation_InstantiateSimpleValue",
    je_1_declarations_multiplevars: "Je_1_Declarations_MultipleVars",
    je_1_declarations_multiplevars_fields: "Je_1_Declarations_MultipleVars_Fields",
    je_1_escapes_1digitoctal_1: "Je_1_Escapes_1DigitOctal_1",
    je_1_escapes_1digitoctal_2: "Je_1_Escapes_1DigitOctal_2",
    je_1_escapes_1digitoctal_3: "Je_1_Escapes_1DigitOctal_3",
    je_1_escapes_1digitoctal_4: "Je_1_Escapes_1DigitOctal_4",
    je_1_escapes_2digitoctal_1: "Je_1_Escapes_2DigitOctal_1",
    je_1_escapes_2digitoctal_2: "Je_1_Escapes_2DigitOctal_2",
    je_1_escapes_2digitoctal_3: "Je_1_Escapes_2DigitOctal_3",
    je_1_escapes_3digitoctal_1: "Je_1_Escapes_3DigitOctal_1",
    je_1_escapes_3digitoctal_2: "Je_1_Escapes_3DigitOctal_2",
    je_1_escapes_3digitoctal_3: "Je_1_Escapes_3DigitOctal_3",
    je_1_escapes_nonexistingescape: "Je_1_Escapes_NonExistingEscape",
    je_1_extends_namedtypearray: "Je_1_Extends_NamedTypeArray",
    je_1_extends_simpletype: "Je_1_Extends_SimpleType",
    je_1_extends_simpletypearray: "Je_1_Extends_SimpleTypeArray",
    je_1_extends_value: "Je_1_Extends_Value",
    je_1_finalfield_noinitializer: "Je_1_FinalField_NoInitializer",
    je_1_for_declarationinupdate: "Je_1_For_DeclarationInUpdate",
    je_1_for_multipledeclarationsininit: "Je_1_For_MultipleDeclarationsInInit",
    je_1_for_multipleupdates: "Je_1_For_MultipleUpdates",
    je_1_for_notastatementinupdate: "Je_1_For_NotAStatementInUpdate",
    je_1_for_primaryexpininit: "Je_1_For_PrimaryExpInInit",
    je_1_for_primaryexpinupdate: "Je_1_For_PrimaryExpInUpdate",
    je_1_for_statementininit: "Je_1_For_StatementInInit",
    je_1_formals_final: "Je_1_Formals_Final",
    je_1_formals_initializer_constructor: "Je_1_Formals_Initializer_Constructor",
    je_1_formals_initializer_method: "Je_1_Formals_Initializer_Method",
    je_1_identifiers_goto: "Je_1_Identifiers_Goto",
    je_1_identifiers_private: "Je_1_Identifiers_Private",
    je_1_implements_namedtypearray: "Je_1_Implements_NamedTypeArray",
    je_1_implements_simpletype: "Je_1_Implements_SimpleType",
    je_1_implements_simpletypearray: "Je_1_Implements_SimpleTypeArray",
    je_1_implements_value: "Je_1_Implements_Value",
    je_1_incdec_incdecnotlvalue: "Je_1_IncDec_IncDecNotLvalue",
    je_1_incdec_parenthesized: "Je_1_IncDec_Parenthesized",
    je_1_instanceinitializers: "Je_1_InstanceInitializers",
    je_1_instanceof_null: "Je_1_InstanceOf_Null",
    je_1_instanceof_primitive: "Je_1_InstanceOf_Primitive",
    je_1_instanceof_void: "Je_1_InstanceOf_Void",
    je_1_interface_constructorabstract: "Je_1_Interface_ConstructorAbstract",
    je_1_interface_constructorbody: "Je_1_Interface_ConstructorBody",
    je_1_interface_field: "Je_1_Interface_Field",
    je_1_interface_finalmethod: "Je_1_Interface_FinalMethod",
    je_1_interface_methodbody: "Je_1_Interface_MethodBody",
    je_1_interface_nobody: "Je_1_Interface_NoBody",
    je_1_interface_staticmethod: "Je_1_Interface_StaticMethod",
    je_1_interface_wrongfilename: "Je_1_Interface_WrongFileName",
    je_1_intrange_minustoobigint: "Je_1_IntRange_MinusTooBigInt",
    je_1_intrange_plustoobigint: "Je_1_IntRange_PlusTooBigInt",
    je_1_intrange_toobigint: "Je_1_IntRange_TooBigInt",
    je_1_intrange_toobigint_ininitializer: "Je_1_IntRange_TooBigInt_InInitializer",
    je_1_intrange_toobigintnegated: "Je_1_IntRange_TooBigIntNegated",
    je_1_joostypes_double: "Je_1_JoosTypes_Double",
    je_1_joostypes_float: "Je_1_JoosTypes_Float",
    je_1_joostypes_long: "Je_1_JoosTypes_Long",
    je_1_labeledstatements: "Je_1_LabeledStatements",
    je_1_literals_class: "Je_1_Literals_Class",
    je_1_literals_exponential: "Je_1_Literals_Exponential",
    je_1_literals_float: "Je_1_Literals_Float",
    je_1_literals_hex: "Je_1_Literals_Hex",
    je_1_literals_long: "Je_1_Literals_Long",
    je_1_literals_octal: "Je_1_Literals_Octal",
    je_1_locals_final: "Je_1_Locals_Final",
    je_1_methods_missingaccessmodifier: "Je_1_Methods_MissingAccessModifier",
    je_1_methods_nonabstractnobody: "Je_1_Methods_NonAbstractNoBody",
    je_1_methods_staticfinal: "Je_1_Methods_StaticFinal",
    je_1_multiarraycreation_assign_2: "Je_1_MultiArrayCreation_Assign_2",
    je_1_multiarraycreation_missingdimension_1: "Je_1_MultiArrayCreation_MissingDimension_1",
    je_1_multiarraycreation_missingdimension_2: "Je_1_MultiArrayCreation_MissingDimension_2",
    je_1_multiarraycreation_missingdimension_4: "Je_1_MultiArrayCreation_MissingDimension_4",
    je_1_multiarraycreation_notype: "Je_1_MultiArrayCreation_NoType",
    je_1_multiarraytypes_dimensions: "Je_1_MultiArrayTypes_Dimensions",
    je_1_neginttoolow: "Je_1_NegIntTooLow",
    je_1_nonjoosconstructs_assignmentoperations_bitwiseand:
        "Je_1_NonJoosConstructs_AssignmentOperations_BitwiseAnd",
    je_1_nonjoosconstructs_assignmentoperations_bitwiseor:
        "Je_1_NonJoosConstructs_AssignmentOperations_BitwiseOr",
    je_1_nonjoosconstructs_assignmentoperations_bitwisexor:
        "Je_1_NonJoosConstructs_AssignmentOperations_BitwiseXOR",
    je_1_nonjoosconstructs_assignmentoperations_divide:
        "Je_1_NonJoosConstructs_AssignmentOperations_Divide",
    je_1_nonjoosconstructs_assignmentoperations_minus:
        "Je_1_NonJoosConstructs_AssignmentOperations_Minus",
    je_1_nonjoosconstructs_assignmentoperations_multiply:
        "Je_1_NonJoosConstructs_AssignmentOperations_Multiply",
    je_1_nonjoosconstructs_assignmentoperations_plus:
        "Je_1_NonJoosConstructs_AssignmentOperations_Plus",
    je_1_nonjoosconstructs_assignmentoperations_remainder:
        "Je_1_NonJoosConstructs_AssignmentOperations_Remainder",
    je_1_nonjoosconstructs_assignmentoperations_shiftleft:
        "Je_1_NonJoosConstructs_AssignmentOperations_ShiftLeft",
    je_1_nonjoosconstructs_assignmentoperations_signshiftright:
        "Je_1_NonJoosConstructs_AssignmentOperations_SignShiftRight",
    je_1_nonjoosconstructs_assignmentoperations_zeroshiftright:
        "Je_1_NonJoosConstructs_AssignmentOperations_ZeroShiftRight",
    je_1_nonjoosconstructs_bitshift_left: "Je_1_NonJoosConstructs_BitShift_Left",
    je_1_nonjoosconstructs_bitshift_signright: "Je_1_NonJoosConstructs_BitShift_SignRight",
    je_1_nonjoosconstructs_bitshift_zeroright: "Je_1_NonJoosConstructs_BitShift_ZeroRight",
    je_1_nonjoosconstructs_bitwise_negation: "Je_1_NonJoosConstructs_Bitwise_Negation",
    je_1_nonjoosconstructs_break: "Je_1_NonJoosConstructs_Break",
    je_1_nonjoosconstructs_choice: "Je_1_NonJoosConstructs_Choice",
    je_1_nonjoosconstructs_continue: "Je_1_NonJoosConstructs_Continue",
    je_1_nonjoosconstructs_dowhile: "Je_1_NonJoosConstructs_DoWhile",
    je_1_nonjoosconstructs_expressionsequence: "Je_1_NonJoosConstructs_ExpressionSequence",
    je_1_nonjoosconstructs_multipletypesprfile: "Je_1_NonJoosConstructs_MultipleTypesPrFile",
    je_1_nonjoosconstructs_nestedtypes: "Je_1_NonJoosConstructs_NestedTypes",
    je_1_nonjoosconstructs_privatefields: "Je_1_NonJoosConstructs_PrivateFields",
    je_1_nonjoosconstructs_privatemethods: "Je_1_NonJoosConstructs_PrivateMethods",
    je_1_nonjoosconstructs_staticinitializers: "Je_1_NonJoosConstructs_StaticInitializers",
    je_1_nonjoosconstructs_strictftp: "Je_1_NonJoosConstructs_Strictftp",
    je_1_nonjoosconstructs_supermethodcall: "Je_1_NonJoosConstructs_SuperMethodCall",
    je_1_nonjoosconstructs_switch: "Je_1_NonJoosConstructs_Switch",
    je_1_nonjoosconstructs_synchronized: "Je_1_NonJoosConstructs_Synchronized",
    je_1_nonjoosconstructs_synchronizedstatement: "Je_1_NonJoosConstructs_SynchronizedStatement",
    je_1_nonjoosconstructs_transient: "Je_1_NonJoosConstructs_Transient",
    je_1_nonjoosconstructs_unaryplus: "Je_1_NonJoosConstructs_UnaryPlus",
    je_1_nonjoosconstructs_unicode: "Je_1_NonJoosConstructs_Unicode",
    je_1_nonjoosconstructs_volatile: "Je_1_NonJoosConstructs_Volatile",
    je_1_packageprivate_class: "Je_1_PackagePrivate_Class",
    je_1_packageprivate_field: "Je_1_PackagePrivate_Field",
    je_1_packageprivate_method: "Je_1_PackagePrivate_Method",
    je_1_superthis_superafterblock: "Je_1_SuperThis_SuperAfterBlock",
    je_1_superthis_superafterstatement: "Je_1_SuperThis_SuperAfterStatement",
    je_1_superthis_superinblock: "Je_1_SuperThis_SuperInBlock",
    je_1_superthis_superinmethod: "Je_1_SuperThis_SuperInMethod",
    je_1_superthis_superthis: "Je_1_SuperThis_SuperThis",
    je_1_superthis_thisafterstatement: "Je_1_SuperThis_ThisAfterStatement",
    je_1_superthis_thisinmethod: "Je_1_SuperThis_ThisInMethod",
    je_1_superthis_twosupercalls: "Je_1_SuperThis_TwoSuperCalls",
    je_1_throw_notexpression: "Je_1_Throw_NotExpression",
    je_1_throws_array: "Je_1_Throws_Array",
    je_1_throws_simpletype: "Je_1_Throws_SimpleType",
    je_1_throws_void: "Je_1_Throws_Void",
    je_1_voidtype_arraycreation: "Je_1_VoidType_ArrayCreation",
    je_1_voidtype_arraydeclaration: "Je_1_VoidType_ArrayDeclaration",
    je_1_voidtype_cast: "Je_1_VoidType_Cast",
    je_1_voidtype_field: "Je_1_VoidType_Field",
    je_1_voidtype_formals: "Je_1_VoidType_Formals",
    je_1_voidtype_local: "Je_1_VoidType_Local",
    je_1_voidtype_voidmethod: "Je_1_VoidType_VoidMethod",
    je_6_assignable_instanceof_simpletypeofsimpletype:
        "Je_6_Assignable_Instanceof_SimpleTypeOfSimpleType",
    je_6_instanceof_primitive_1: "Je_6_InstanceOf_Primitive_1",
    je_6_instanceof_primitive_2: "Je_6_InstanceOf_Primitive_2",
    je_native: "Je_Native",
    je_throws: "Je_Throws",
}
