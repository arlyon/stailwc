use swc_ecma_transforms_testing::test;

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor),
    basic,
    // Input codes
    r#"<Test tw="h-4" />"#,
    // Output codes after transformed with plugin
    r#"console.log("transform");"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor),
    string,
    // Input codes
    r#"<Test tw={"h-4"} />"#,
    // Output codes after transformed with plugin
    r#"console.log("transform");"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor),
    variable,
    // Input codes
    r#"<Test tw={variable} />"#,
    // Output codes after transformed with plugin
    r#"console.log("transform");"#
);
