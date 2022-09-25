#![feature(box_patterns)]
#![deny(clippy::unwrap_used)]
// bug in swc
#![allow(clippy::not_unsafe_ptr_arg_deref)]
mod config;
mod parse;
mod plugin;
#[cfg(test)]
mod test;
mod util;

use config::TailwindConfig;
use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            ArrayLit, Expr, ExprOrSpread, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread,
            JSXAttrValue, JSXExpr, JSXExprContainer, JSXOpeningElement, Lit, ObjectLit, Program,
            TaggedTpl, TplElement,
        },
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

use crate::config::AppConfig;
use parse::{from::literal_from_directive, nom::Directive};

#[derive(Default)]
pub struct TransformVisitor<'a> {
    config: TailwindConfig<'a>,

    tw_attr: Option<ObjectLit>,
    tw_tpl: Option<ObjectLit>,
}

impl<'a> TransformVisitor<'a> {
    pub fn new(config: TailwindConfig<'a>) -> Self {
        Self {
            config,
            tw_attr: None,
            tw_tpl: None,
        }
    }
}

/**
 * Implement necessary visit_mut_* methods for actual custom transform.
 * A comprehensive list of possible visitor methods can be found here:
 * https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
 */
impl<'a> VisitMut for TransformVisitor<'a> {
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
                let d = match Directive::parse(&string.value) {
                    Ok((_, d)) => d,
                    Err(e) => {
                        println!("fail : could not parse `{}`  {}", string.value, e);
                        return;
                    },
                };
                if self.tw_attr.replace(literal_from_directive(d, &self.config)).is_some() {
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

        let lit = match self.tw_attr.take() {
            Some(v) => v,
            _ => return,
        };

        n.attrs.retain(|attr| {
            !matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr {
                            name: JSXAttrName::Ident(Ident { sym, .. }),
                           ..
                       }) if sym == "tw")
        });

        let css_attr = n.attrs.iter_mut().find_map(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(Ident { sym, .. }),
                value,
                ..
            }) if sym == "css" => value.as_mut(),
            _ => None,
        });

        // todo(arlyon): handle function

        if let Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
            expr: JSXExpr::Expr(box e),
            ..
        })) = css_attr
        {
            match e {
                // if the expr is an array, push our tailwind styles to the end
                Expr::Array(a) => a.elems.push(Some(ExprOrSpread {
                    expr: Box::new(Expr::Object(lit)),
                    spread: None,
                })),
                // for anything else, convert it to an array and push our tailwind styles to the end
                _ => {
                    *e = Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: vec![
                            Some(ExprOrSpread {
                                expr: Box::new(e.take()),
                                spread: None,
                            }),
                            Some(ExprOrSpread {
                                expr: Box::new(Expr::Object(lit)),
                                spread: None,
                            }),
                        ],
                    });
                }
            }
        } else {
            // if the attr doesn't exist, push one
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
        };
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

        let d = match Directive::parse(text) {
            Ok((_, d)) => d,
            Err(e) => {
                println!("fail : could not parse `{}` - {}", text, e);
                return;
            }
        };
        if self
            .tw_tpl
            .replace(literal_from_directive(d, &self.config))
            .is_some()
        {
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
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = metadata
        .get_transform_plugin_config()
        .expect("failed to load config");

    let deser = &mut serde_json::Deserializer::from_str(&config);
    let config: Result<AppConfig, _> = serde_path_to_error::deserialize(deser);

    match config {
        Ok(config) => program.fold_with(&mut as_folder(TransformVisitor::new(config.config))),
        Err(error) => {
            println!(
                "fail : could not read tailwind config at {}: {}",
                error.path(),
                error.inner()
            );
            program
        }
    }
}
