extern crate juicyj;

macro_rules! feature_tests {
    ($($name:ident: $case:tt,)*) => {
    $(
        #[test]
        fn $name() {
            let filename: String = format!("tests/cases/special/{}.java", $case);
            let src: String = juicyj::scanner::read_src_file(&filename);

            juicyj::scanner::tests::scan_or_assert(&filename, &src);
        }
    )*
    }
}

feature_tests! {
    implicitly_abstract_methods: "ImplicitlyAbstractMethods",
}
