extern crate juicyj;

macro_rules! a2_pass_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let filename: String = format!("tests/cases/a2/pass/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

macro_rules! a2_pass_tests_folders {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        #[ignore]
        fn $name() {
            let paths = std::fs::read_dir(format!("tests/cases/a2/pass/{}", $case)).unwrap();
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

a2_pass_tests! {
    j1_1_cast_namedtypeasvariable: "J1_1_Cast_NamedTypeAsVariable",
    j1_2_fields_case: "J1_2_Fields_Case",
    j1_2_locals_overlapping_afterblock: "J1_2_Locals_Overlapping_AfterBlock",
    j1_4_constructor_duplicatearraytypes: "J1_4_Constructor_DuplicateArrayTypes",
    j1_4_constructor_matchassets: "J1_4_Constructor_MatchAsSets",
    j1_4_duplicatemethoddeclare_methodnameequalsconstructorname:
        "J1_4_DuplicateMethodDeclare_MethodNameEqualsConstructorName",
    j1_4_methoddeclare_duplicatearraytypes: "J1_4_MethodDeclare_DuplicateArrayTypes",
    j1_4_override_finaloverridenonfinal: "J1_4_Override_FinalOverrideNonFinal",
    j1_4_override_publicoverridesprotected: "J1_4_Override_PublicOverridesProtected",
    j1_access_override2: "J1_access_override2",
    j1_arbitrarylocaldeclaration: "J1_arbitrarylocaldeclaration",
    j1_assign_object_to_object: "J1_assign_Object_to_Object",
    j1_cast_to_same_type: "J1_cast_to_same_type",
    j1_classextendsobject1: "J1_classextendsobject1",
    j1_classextendsobject2: "J1_classextendsobject2",
    j1_constructorwithsamenameasmethod: "J1_constructorWithSameNameAsMethod",
    j1_formal_with_same_name_as_field: "J1_formal_with_same_name_as_field",
    j1_formalindex: "J1_formalindex",
    j1_importname10: "J1_importName10",
    j1_importname11: "J1_importName11",
    j1_importname9: "J1_importName9",
    j1_inherited_hashcode: "J1_inherited_hashcode",
    j1_local_duplicate: "J1_local_duplicate",
    j1_localvariablescope: "J1_localvariablescope",
    j1_noduplicatefield: "J1_noduplicatefield",
}

a2_pass_tests_folders! {
    j1_3_importondemand_defaultimportinpresenceofotherimport:
        "J1_3_ImportOnDemand_DefaultImportInPresenceOfOtherImport",
    j1_3_importondemand_programdefinedpackage: "J1_3_ImportOnDemand_ProgramDefinedPackage",
    j1_3_infixresolvestotype: "J1_3_InfixResolvesToType",
    j1_3_ondemandimport_nonambiguous_default: "J1_3_OnDemandImport_NonAmbiguous_Default",
    j1_3_ondemandimport_nonambiguous_samepackage: "J1_3_OnDemandImport_NonAmbiguous_SamePackage",
    j1_3_packageclashwithtype_linked_mutated: "J1_3_PackageClashWithType_Linked_Mutated",
    j1_3_packagedecl_multiplefilesinsamepackage: "J1_3_PackageDecl_MultipleFilesInSamePackage",
    j1_3_packagedecl_samepackageandclassname: "J1_3_PackageDecl_SamePackageAndClassName",
    j1_3_packageexists_asprefix_external: "J1_3_PackageExists_AsPrefix_External",
    j1_3_packageexists_asprefix_internal: "J1_3_PackageExists_AsPrefix_Internal",
    j1_3_resolve_linktocorrectpackage: "J1_3_Resolve_LinkToCorrectPackage",
    j1_3_resolve_packageprefixmatchclassname: "J1_3_Resolve_PackagePrefixMatchClassName",
    j1_3_resolve_samepackage_external: "J1_3_Resolve_SamePackage_External",
    j1_3_singletypeimport_clashwithondemand: "J1_3_SingleTypeImport_ClashWithOnDemand",
    j1_3_singletypeimport_clashwithpackagename: "J1_3_SingleTypeImport_ClashWithPackageName",
    j1_3_singletypeimport_importprogramclass: "J1_3_SingleTypeImport_ImportProgramClass",
    j1_3_singletypeimport_importself: "J1_3_SingleTypeImport_ImportSelf",
    j1_3_singletypeimport_multiplefromsamepackage: "J1_3_SingleTypeImport_MultipleFromSamePackage",
    j1_3_singletypeimport_multipleimportsofsametype:
        "J1_3_SingleTypeImport_MultipleImportsOfSameType",
    j1_3_singletypeimport_noclash: "J1_3_SingleTypeImport_NoClash",
    j1_4_abstractmethod_inheritabstractfromobject: "J1_4_AbstractMethod_InheritAbstractFromObject",
    j1_4_abstractmethod_inheritedfrominterface: "J1_4_AbstractMethod_InheritedFromInterface",
    j1_4_classextendsclass_samename: "J1_4_ClassExtendsClass_SameName",
    j1_4_classimplementsinterface_multipletimes: "J1_4_ClassImplementsInterface_MultipleTimes",
    j1_4_inheritedfields_samefield_twoways: "J1_4_InheritedFields_SameField_TwoWays",
    j1_4_interfacemethod_fromobject: "J1_4_InterfaceMethod_FromObject",
    j1_4_packageclashwithtype_loaded: "J1_4_PackageClashWithType_Loaded",
    j1_4_packageclashwithtype_notloaded: "J1_4_PackageClashWithType_NotLoaded",
    j1_4_packageclashwithtype_singletypeimport_defaultpackage:
        "J1_4_PackageClashWithType_SingleTypeImport_DefaultPackage",
    j1_4_packagenameisclassname_defaultpackage: "J1_4_PackageNameIsClassName_DefaultPackage",
    j1_4_resolve_notdefaultpackage: "J1_4_Resolve_NotDefaultPackage",
    j1_4_singletypeimport_ondemandsclash: "J1_4_SingleTypeImport_OnDemandsClash",
    j1_6_protectedaccess_staticmethod_this: "J1_6_ProtectedAccess_StaticMethod_This",
    j1_abstract: "J1_abstract",
    j1_classimplementsserializable1: "J1_classimplementsserializable1",
    j1_classimplementsserializable2: "J1_classimplementsserializable2",
    j1_classimport: "J1_classimport",
    j1_fields: "J1_fields",
    j1_final_method_override1: "J1_final_method_override1",
    j1_hierachycheck14: "J1_hierachyCheck14",
    j1_hierachycheck28: "J1_hierachyCheck28",
    j1_hierachycheck29: "J1_hierachyCheck29",
    j1_hierachycheck31: "J1_hierachyCheck31",
    j1_implicitsuper: "J1_implicitsuper",
    j1_importnamelookup1: "J1_importNameLookup1",
    j1_importnamelookup2: "J1_importNameLookup2",
    j1_importnamelookup3: "J1_importNameLookup3",
    j1_importnamelookup4: "J1_importNameLookup4",
    j1_importnamelookup5: "J1_importNameLookup5",
    j1_importnamelookup6: "J1_importNameLookup6",
    j1_importnamelookup7: "J1_importNameLookup7",
    j1_instance_method_hide1: "J1_instance_method_hide1",
    j1_interfaceassignable: "J1_interfaceassignable",
    j1_interfaceobject: "J1_InterfaceObject",
    j1_name: "J1_name",
    j1_on_demand_imports_clash: "J1_on_demand_imports_clash",
    j1_package: "J1_package",
    j1_packageimport: "J1_packageimport",
    j1_public_method_protected_override1: "J1_public_method_protected_override1",
    j1_resolvetype2: "J1_resolvetype2",
    j1_resolvetype3: "J1_resolvetype3",
    j1_resolvetype4: "J1_resolvetype4",
    j1_resolvetype6: "J1_resolvetype6",
    j1_samepackage: "J1_samePackage",
    j1_singletypeimport: "J1_singleTypeImport",
    j1_singletypeimportsametypemultipletimes: "J1_singleTypeImportSameTypeMultipleTimes",
    j1_static_method_override1: "J1_static_method_override1",
    j1_subtype1: "J1_SubType1",
    j1_subtype2: "J1_SubType2",
    j1_supermethod_override1: "J1_supermethod_override1",
    j1_supermethod_override2: "J1_supermethod_override2",
    j1_supermethod_override3: "J1_supermethod_override3",
    j1_supermethod_override4: "J1_supermethod_override4",
    j1_supermethod_override5: "J1_supermethod_override5",
    j1_supermethod_override6: "J1_supermethod_override6",
    j1_typecheck_assignment: "J1_typecheck_assignment",
    j2_3_singletypeimport_importself_interface: "J2_3_SingleTypeImport_ImportSelf_Interface",
    j2_4_implementsinterface_twicebyname: "J2_4_ImplementsInterface_TwiceByName",
    j2_4_interfaceextends_multipleways: "J2_4_InterfaceExtends_MultipleWays",
    j2_hierachycheck22: "J2_hierachyCheck22",
    j2_hierachycheck23: "J2_hierachyCheck23",
    j2_hierachycheck24: "J2_hierachyCheck24",
    j2_hierachycheck25: "J2_hierachyCheck25",
    j2_ifaceimplicitabstract: "J2_Ifaceimplicitabstract",
    j2_interface1: "J2_Interface1",
    j2_interface10: "J2_Interface10",
    j2_interface11: "J2_Interface11",
    j2_interface2: "J2_Interface2",
    j2_interface3: "J2_Interface3",
    j2_interface6: "J2_Interface6",
    j2_interface7: "J2_Interface7",
    j2_interface8: "J2_Interface8",
    j2_interface9: "J2_Interface9",
    j2_interface_omitted_abstract: "J2_interface_omitted_abstract",
}
