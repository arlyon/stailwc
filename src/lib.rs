#![feature(box_patterns)]

use swc_core::{
    ast::{
        Expr, Ident, JSXAttr, JSXAttrName, JSXAttrValue, JSXExpr, JSXExprContainer, Lit, Program,
        Str,
    },
    common::{errors::HANDLER, MultiSpan},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    testing_transform::test,
    visit::{as_folder, FoldWith, VisitMut},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        let sym = match &n.name {
            JSXAttrName::Ident(Ident { sym, .. }) if sym == "tw" => sym,
            _ => return,
        };

        match &n.value {
            Some(JSXAttrValue::Lit(Lit::Str(Str { value, .. })))
            | Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Lit(Lit::Str(Str { value, .. }))),
                ..
            })) => {
                println!("match! {}={}", sym, value)
            }
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Ident(_)),
                span,
                ..
            })) => {
                HANDLER.with(|h| {
                    h.struct_span_warn(MultiSpan::from_span(*span), "variables are not supported")
                        .emit()
                });
            }
            _ => println!("fail! {:#?}", n),
        }
        // match n
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

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
