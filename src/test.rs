use swc_ecma_transforms_testing::test;
use swc_ecma_visit::as_folder;

use crate::TransformVisitor;

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    basic,
    // Input codes
    r#"<Test tw="h-4" />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{"height": "4em"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    string,
    // Input codes
    r#"<Test tw={"h-4"} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{"height": "4em"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    template,
    r#"const x = tw`h-4`"#,
    r#"const x = {"height": "4em"}"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    template_jsx,
    r#"<Test css={tw`h-4`} />"#,
    r#"<Test css={{"height": "4em"}} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    existing_css_object,
    // Input codes
    r#"<Test tw="h-4" css={{width: "4em"}} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{width: "4em"}, {"height": "4em"}]} />"#
);

test!(
    swc_ecma_parser::Syntax::Typescript(swc_ecma_parser::TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor::default()),
    existing_css_array,
    // Input codes
    r#"<Test tw="h-4" css={[]} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{"height": "4em"}]} />"#
);
