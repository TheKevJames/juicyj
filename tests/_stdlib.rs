extern crate juicyj;

macro_rules! feature_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("stdlib/java/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

feature_tests! {
    io_outputstream: "io/OutputStream",
    io_printstream: "io/PrintStream",
    io_serializable: "io/Serializable",
    lang_boolean: "lang/Boolean",
    lang_byte: "lang/Byte",
    lang_character: "lang/Character",
    lang_class: "lang/Class",
    lang_cloneable: "lang/Cloneable",
    lang_integer: "lang/Integer",
    lang_number: "lang/Number",
    lang_object: "lang/Object",
    lang_short: "lang/Short",
    lang_string: "lang/String",
    lang_system: "lang/System",
    util_arrays: "util/Arrays",
}
