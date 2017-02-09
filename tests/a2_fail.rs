extern crate juicyj;

macro_rules! a2_fail_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let filename: String = format!("tests/cases/a2/fail/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_and_assert(&filename, &src);
        }
    )*
    }
}

macro_rules! a2_fail_tests_folders {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let paths = std::fs::read_dir(format!("tests/cases/a2/fail/{}", $case)).unwrap();
            for path in paths {
                match path.unwrap().path().to_str() {
                    Some(filename) => {
                        // TODO: compile multiple together
                        let src: String = juicyj::scanner::read_src_file(&filename.to_string());
                        juicyj::scanner::tests::scan_and_assert(&filename, &src);
                    }
                    _ => (),
                }
            }
        }
    )*
    }
}

a2_fail_tests! {
    je_12_fields_staticnonstatic: "Je_12_Fields_StaticNonStatic",
    je_4_duplicatemethoddeclare_noargs: "Je_4_DuplicateMethodDeclare_NoArgs",
    je_4_extendfinal: "Je_4_ExtendFinal",
    je_14_interface_duplicatemethoddeclare: "Je_14_Interface_DuplicateMethodDeclare",
    je_4_finaloverride_differentreturntypes: "Je_4_FinalOverride_DifferentReturnTypes",
    je_4_finaloverride_samesignature: "Je_4_FinalOverride_SameSignature",
    je_14_interface_selfdependency_extendsitself: "Je_14_Interface_SelfDependency_ExtendsItself",
    je_4_implementnoninterface_class: "Je_4_ImplementNonInterface_Class",
    je_2_constructorparameter_duplicate: "Je_2_ConstructorParameter_Duplicate",
    je_2_constructorparameter_overlapping: "Je_2_ConstructorParameter_Overlapping",
    je_2_fields_differentaccess: "Je_2_Fields_DifferentAccess",
    je_2_fields_differenttypes: "Je_2_Fields_DifferentTypes",
    je_3_resolve_nonexistingsuperclass: "Je_3_Resolve_NonExistingSuperclass",
    je_2_fields_multiplefields: "Je_2_Fields_MultipleFields",
    je_3_resolve_samepackageandclassname: "Je_3_Resolve_SamePackageAndClassName",
    je_2_locals_overlapping_deeplynested: "Je_2_Locals_Overlapping_DeeplyNested",
    je_2_locals_overlapping_forinitializer: "Je_2_Locals_Overlapping_ForInitializer",
    je_2_locals_overlapping_inconditionalelse: "Je_2_Locals_Overlapping_InConditionalElse",
    je_2_locals_overlapping_inconditionalthen: "Je_2_Locals_Overlapping_InConditionalThen",
    je_2_locals_overlapping_insidedoubleblock: "Je_2_Locals_Overlapping_InsideDoubleBlock",
    je_2_locals_overlapping_insideloop: "Je_2_Locals_Overlapping_InsideLoop",
    je_2_locals_overlapping_insidenewblock: "Je_2_Locals_Overlapping_InsideNewBlock",
    je_2_locals_overlapping_samelevel: "Je_2_Locals_Overlapping_SameLevel",
    je_2_locals_overlapping_sameline: "Je_2_Locals_Overlapping_SameLine",
    je_2_parameter_abstractdeclaredtwice: "Je_2_Parameter_AbstractDeclaredTwice",
    je_4_abstractmethod_declared: "Je_4_AbstractMethod_Declared",
    je_2_parameter_overlappingwithlocalinconditional:
        "Je_2_Parameter_OverlappingWithLocalInConditional",
    je_2_parameter_overlappingwithlocalinloop: "Je_2_Parameter_OverlappingWithLocalInLoop",
    je_2_parameter_overlappingwithlocalinsidenewblock:
        "Je_2_Parameter_OverlappingWithLocalInsideNewBlock",
    je_2_parameter_overlappingwithlocal: "Je_2_Parameter_OverlappingWithLocal",
    je_4_replaceinstance_fromsuperclass: "Je_4_ReplaceInstance_FromSuperclass",
    je_2_parameter_overlappingwithlocalnotfirst: "Je_2_Parameter_OverlappingWithLocalNotFirst",
    je_2_parameter_overlappingwithparameter: "Je_2_Parameter_OverlappingWithParameter",
    je_4_duplicateconstructor_args: "Je_4_DuplicateConstructor_Args",
    je_4_duplicateconstructor_arrayargs: "Je_4_DuplicateConstructor_ArrayArgs",
    je_3_importondemand_nonexisting: "Je_3_ImportOnDemand_NonExisting",
    je_4_duplicateconstructor_noargs: "Je_4_DuplicateConstructor_NoArgs",
    je_4_selfdependency_extendsitself: "Je_4_SelfDependency_ExtendsItself",
    je_3_importondemand_nonexistingpackage_fromprevioustestcase:
        "Je_3_ImportOnDemand_NonExistingPackage_FromPreviousTestcase",
    je_4_duplicatemethoddeclare_args: "Je_4_DuplicateMethodDeclare_Args",
    je_5_interface_implicitreplace_differentreturntype:
        "Je_5_Interface_ImplicitReplace_DifferentReturnType",
    je_4_duplicatemethoddeclare_arrayargs: "Je_4_DuplicateMethodDeclare_ArrayArgs",
    je_4_duplicatemethoddeclare_differentreturntypes:
        "Je_4_DuplicateMethodDeclare_DifferentReturnTypes",
}

a2_fail_tests_folders! {
    je_3_packageclashwithtype_linked: "Je_3_PackageClashWithType_Linked",
    je_13_interface_singleimport_clashwithclass: "Je_13_Interface_SingleImport_ClashWithClass",
    je_3_packageclashwithtype_singletypeimport: "Je_3_PackageClashWithType_SingleTypeImport",
    je_14_interface_declarestostring_differentreturntype:
        "Je_14_Interface_DeclaresToString_DifferentReturnType",
    je_3_packageexists_almostprefix_external: "Je_3_PackageExists_AlmostPrefix_External",
    je_4_extendnonclass: "Je_4_ExtendNonClass",
    je_14_interface_declarestostring_throwsconflict:
        "Je_14_Interface_DeclaresToString_ThrowsConflict",
    je_3_packageexists_almostprefix_internal: "Je_3_PackageExists_AlmostPrefix_Internal",
    je_4_finalhide: "Je_4_FinalHide",
    je_3_packagenameisclassname: "Je_3_PackageNameIsClassName",
    je_14_interface_implicitpublicmethod_protectedoverride:
        "Je_14_Interface_ImplicitPublicMethod_ProtectedOverride",
    je_3_packagenameisclassname_external: "Je_3_PackageNameIsClassName_External",
    je_14_interface_selfdependency_cyclicextend: "Je_14_Interface_SelfDependency_CyclicExtend",
    je_3_packagenameisclassname_externalprefix: "Je_3_PackageNameIsClassName_ExternalPrefix",
    je_4_hide_differentreturntypes: "Je_4_Hide_DifferentReturnTypes",
    je_3_packagenameisclassname_prefix: "Je_3_PackageNameIsClassName_Prefix",
    je_3_resolve_implicitjavaio: "Je_3_Resolve_ImplicitJavaIO",
    je_4_implementnoninterface_interfaceandclass: "Je_4_ImplementNonInterface_InterfaceAndClass",
    je_3_resolve_importdifferentfromsamepackage: "Je_3_Resolve_ImportDifferentFromSamePackage",
    je_4_implementtwice_qualifiedname: "Je_4_ImplementTwice_QualifiedName",
    je_2_duplicatetype: "Je_2_DuplicateType",
    je_3_resolve_linktocorrectpackage: "Je_3_Resolve_LinkToCorrectPackage",
    je_4_implementtwice_simplename: "Je_4_ImplementTwice_SimpleName",
    je_3_resolve_missingimport: "Je_3_Resolve_MissingImport",
    je_4_inheritshadowsnonabstract: "Je_4_InheritShadowsNonabstract",
    je_4_interfaceextendscyclicinterface: "Je_4_InterfaceExtendsCyclicInterface",
    je_4_interface_finalmethodfromobject: "Je_4_Interface_FinalMethodFromObject",
    je_3_singletypeimport_clashwithclass: "Je_3_SingleTypeImport_ClashWithClass",
    je_4_override_differentreturntypes_abstractfromsuperclassandinterface:
        "Je_4_Override_DifferentReturnTypes_AbstractFromSuperclassAndInterface",
    je_3_singletypeimport_clashwithclass_inpackage:
        "Je_3_SingleTypeImport_ClashWithClass_InPackage",
    je_4_override_differentreturntypesfrominterface:
        "Je_4_Override_DifferentReturnTypesFromInterface",
    je_3_singletypeimport_clashwitheachother: "Je_3_SingleTypeImport_ClashWithEachOther",
    je_4_override_differentreturntypes_fromsuperclassandinterface:
        "Je_4_Override_DifferentReturnTypes_FromSuperclassAndInterface",
    je_3_singletypeimport_clashwitheachother_multipleimports:
        "Je_3_SingleTypeImport_ClashWithEachOther_MultipleImports",
    je_4_override_differentreturntypes_fromsuperclassandinterface_nonvoid:
        "Je_4_Override_DifferentReturnTypes_FromSuperclassAndInterface_NonVoid",
    je_3_singletypeimport_clashwithinterface: "Je_3_SingleTypeImport_ClashWithInterface",
    je_4_override_differentreturntypes_twointerfaces:
        "Je_4_Override_DifferentReturnTypes_TwoInterfaces",
    je_3_singletypeimport_nonexistingpackage: "Je_3_SingleTypeImport_NonExistingPackage",
    je_4_protectedhide_fromsuperclass: "Je_4_ProtectedHide_FromSuperclass",
    je_3_singletypeimport_nonexistingtype: "Je_3_SingleTypeImport_NonExistingType",
    je_4_protectedoverride_abstract: "Je_4_ProtectedOverride_Abstract",
    je_3_undefinedtype_defaultpackagenotvisible: "Je_3_UndefinedType_DefaultPackageNotVisible",
    je_4_protectedoverride_differentthrows: "Je_4_ProtectedOverride_DifferentThrows",
    je_4_abstractmethod_abstractobjectmethods: "Je_4_AbstractMethod_AbstractObjectMethods",
    je_4_protectedoverride_exception_clone: "Je_4_ProtectedOverride_Exception_Clone",
    je_4_protectedoverride_frominterface: "Je_4_ProtectedOverride_FromInterface",
    je_4_abstractmethod_inheritfrominterface_1: "Je_4_AbstractMethod_InheritFromInterface_1",
    je_4_protectedoverride_fromsuperclass: "Je_4_ProtectedOverride_FromSuperclass",
    je_4_abstractmethod_inheritfrominterface_2: "Je_4_AbstractMethod_InheritFromInterface_2",
    je_4_protectedoverride_fromsuperclassandinterface:
        "Je_4_ProtectedOverride_FromSuperclassAndInterface",
    je_4_abstractmethod_inheritfromsuperclass: "Je_4_AbstractMethod_InheritFromSuperclass",
    je_4_protectedoverride_twoversionsfromsuperclass:
        "Je_4_ProtectedOverride_TwoVersionsFromSuperclass",
    je_4_abstractmethod_inheritfromsuperclassinterface:
        "Je_4_AbstractMethod_InheritFromSuperclassInterface",
    je_4_abstractmethod_inheritfromsuperinterface:
        "Je_4_AbstractMethod_InheritFromSuperInterface",
    je_4_replacestatic_fromsuperclass: "Je_4_ReplaceStatic_FromSuperclass",
    je_4_abstractmethod_notallimplemented: "Je_4_AbstractMethod_NotAllImplemented",
    je_4_replacestatic_fromsuperclass_differentreturntypes:
        "Je_4_ReplaceStatic_FromSuperclass_DifferentReturnTypes",
    je_3_importondemand_clashwithimplicitimport: "Je_3_ImportOnDemand_ClashWithImplicitImport",
    je_4_classextendscyclicclass: "Je_4_ClassExtendsCyclicClass",
    je_4_resolve_defaultpackage: "Je_4_Resolve_DefaultPackage",
    je_3_importondemand_classinmultiplepackages: "Je_3_ImportOnDemand_ClassInMultiplePackages",
    je_4_selfdependency_circularextends_1: "Je_4_SelfDependency_CircularExtends_1",
    je_3_importondemand_classnameaspackage: "Je_3_ImportOnDemand_ClassNameAsPackage",
    je_4_selfdependency_circularextends_2: "Je_4_SelfDependency_CircularExtends_2",
    je_3_importondemand_packageprefixexists: "Je_3_ImportOnDemand_PackagePrefixExists",
    je_3_packageclashwithtype_explicit: "Je_3_PackageClashWithType_Explicit",
}
