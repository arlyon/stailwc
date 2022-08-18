#![feature(box_patterns)]

mod infer;
mod tailwind_parse;
#[cfg(test)]
mod test;

use swc_ecmascript::ast::{
    Expr, Ident, JSXAttr, JSXAttrName, JSXAttrValue, JSXExpr, JSXExprContainer, Lit, Program,
};

use swc_common::DUMMY_SP;
use swc_ecma_visit::{
    as_folder,
    swc_ecma_ast::{JSXAttrOrSpread, JSXOpeningElement, ObjectLit, TaggedTpl, TplElement},
    FoldWith, VisitMut, VisitMutWith,
};
use swc_plugin::metadata::TransformPluginProgramMetadata;
use swc_plugin_macro::plugin_transform;
use tailwind_parse::Directive;

#[derive(Default)]
pub struct TransformVisitor {
    css_attr: Option<ObjectLit>,
    tw_tpl: Option<ObjectLit>,
}

/**
 * Implement necessary visit_mut_* methods for actual custom transform.
 * A comprehensive list of possible visitor methods can be found here:
 * https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
 */
impl VisitMut for TransformVisitor {
    /**
     * Handle jsx attributes and convert them into emotion.
     */
    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        let _sym = match &n.name {
            JSXAttrName::Ident(Ident { sym, .. }) if sym == "tw" => "tw",
            _ => {
                n.visit_mut_children_with(self);
                return;
            }
        };

        match &n.value {
            // tw="h-4"
            Some(JSXAttrValue::Lit(Lit::Str(string)))
            // tw={"h-4"}
            | Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Lit(Lit::Str(string))),
                ..
            })) => {
                let d = Directive::from(&*string.value);
                if self.css_attr.replace(d.parse()).is_some() {
                    println!("warn : encountered multiple tw attributes");
                }
            }
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Ident(Ident { sym, .. })),
                ..
            })) => {
                // todo(arlyon): enable error reporting
                // HANDLER.with(|h| {
                //         h.struct_span_warn(MultiSpan::from_span(*span), "variables are not supported")
                //             .emit()
                // });
                println!("fail : variables are not supported `{}`", sym);
            }
            _ => println!("fail : unknown pattern {:#?}", n),
        }
    }

    /**
     * Visit children to extract any tailwind attributes, then:
     * - convert them into emotion
     * - remove any tailwind attributes
     */
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        n.attrs.visit_mut_children_with(self);

        let lit = match self.css_attr.take() {
            Some(v) => v,
            _ => return,
        };

        n.attrs.retain(|attr| {
            !matches!(attr, swc_ecma_visit::swc_ecma_ast::JSXAttrOrSpread::JSXAttr(JSXAttr {
                            name: JSXAttrName::Ident(Ident { sym, .. }),
                           ..
                       }) if sym == "tw")
        });

        // todo(arlyon): handle array, function, object merge

        n.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            name: JSXAttrName::Ident(Ident {
                sym: "css".into(),
                span: DUMMY_SP,
                optional: false,
            }),
            span: DUMMY_SP,
            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(Box::new(Expr::Object(lit))),
                span: DUMMY_SP,
            })),
        }));
    }

    /**
     * On discovery of a template tag, if it is a tailwind template tag,
     * convert it to an emotion object.
     */
    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        let _sym = match &n.tag {
            box Expr::Ident(Ident { sym, .. }) if sym == "tw" => "tw",
            _ => {
                n.visit_mut_children_with(self);
                return;
            }
        };

        let text = match n.tpl.quasis.as_slice() {
            [TplElement { raw, .. }] => raw,
            _ => {
                println!("fail : did not expect multiple quasis. please file an issue");
                return;
            }
        };

        let d = Directive::from(text.as_ref());
        if self.tw_tpl.replace(d.parse()).is_some() {
            println!("warn : encountered bad state in template tag");
        }
    }

    /**
     * Visit an expression, optionally substituting the template tag with the
     * generated emotion object.
     */
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);
        if let Some(objlit) = self.tw_tpl.take() {
            *n = Expr::Object(objlit);
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor::default()))
}
