use swc_ecma_transforms_testing::test;
use swc_ecma_visit::as_folder;

use crate::TransformVisitor;

fn test_visitor() -> TransformVisitor<'static> {
    let mut t = TransformVisitor::default();

    t.config.theme.spacing.insert("4", "1rem");

    t
}

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    basic,
    // Input codes
    r#"<Test tw="h-4" />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{"height": "1rem"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    string,
    // Input codes
    r#"<Test tw={"h-4"} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{"height": "1rem"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    template,
    r#"const x = tw`h-4`"#,
    r#"const x = {"height": "1rem"}"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    template_jsx,
    r#"<Test css={tw`h-4`} />"#,
    r#"<Test css={{"height": "1rem"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    existing_css_object,
    // Input codes
    r#"<Test tw="h-4" css={{width: "1rem"}} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{width: "1rem"}, {"height": "1rem"}]} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    existing_css_array,
    // Input codes
    r#"<Test tw="h-4" css={[]} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{"height": "1rem"}]} />"#
);
