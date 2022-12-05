#![feature(box_patterns)]
#![feature(let_chains)]
#![feature(drain_filter)]
#![deny(clippy::unwrap_used)]
// bug in swc
#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod css;
#[cfg(test)]
mod test;

use nom_locate::LocatedSpan;
use swc_core::{
    common::{
        errors::{DiagnosticBuilder, Handler},
        util::take::Take,
        Span, DUMMY_SP,
    },
    ecma::{
        ast::{
            ArrayLit, CallExpr, Callee, Expr, ExprOrSpread, Ident, ImportDecl, JSXAttr,
            JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName, JSXExpr, JSXExprContainer,
            JSXOpeningElement, Lit, MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem,
            ObjectLit, Program, Str, TaggedTpl, Tpl, TplElement,
        },
        atoms::Atom,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{errors::HANDLER, plugin_transform, proxies::TransformPluginProgramMetadata},
};
use tailwind_config::TailwindConfig;
use tailwind_parse::Directive;

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig<'a> {
    #[serde(borrow)]
    pub config: TailwindConfig<'a>,

    /// Strict mode throws an error when an unknown class is used.
    #[serde(default)]
    pub strict: bool,
}

#[derive(Debug)]
enum TplTransform {
    /// tw`...`
    Style(ObjectLit),
    /// tw.div`...`
    Component(Ident, ObjectLit),
    /// tw(Component)`...`
    ComponentCustom(Vec<ExprOrSpread>, ObjectLit),
}

#[derive(Default, Debug)]
pub struct TransformVisitor<'a> {
    config: TailwindConfig<'a>,
    strict: bool,
    /// This is treated as a stack because tw attrs can be nested
    tw_attr_stack: DepthStack<(Span, ObjectLit)>,
    tw_tpl: Option<TplTransform>,
    tw_style_imported: bool,
}

/// A stack that only allows a single item to be pushed at a given depth.
#[derive(Debug)]
struct DepthStack<T> {
    stack: Vec<(usize, T)>,
    depth: usize,
}

impl<T> DepthStack<T> {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            depth: 0,
        }
    }

    fn push(&mut self, item: T) -> Option<T> {
        if self
            .stack
            .last()
            .filter(|(d, _)| *d == self.depth)
            .is_some()
        {
            return self.stack.pop().map(|(_, v)| v);
        }

        self.stack.push((self.depth, item));
        None
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop().map(|(_, item)| item)
    }

    fn peek(&self) -> Option<&T> {
        self.stack
            .last()
            .filter(|(d, _)| *d == self.depth)
            .map(|(_, item)| item)
    }

    fn depth(&self) -> usize {
        self.depth
    }

    #[tracing::instrument(skip(self))]
    fn inc_depth(&mut self) {
        self.depth += 1;
        tracing::trace!(depth = self.depth);
    }

    #[tracing::instrument(skip(self))]
    fn dec_depth(&mut self) {
        self.depth -= 1;
        tracing::trace!(depth = self.depth);
        self.stack.retain(|(d, _)| *d <= self.depth);
    }
}

impl<T> Default for DepthStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TransformVisitor<'a> {
    pub fn new(config: AppConfig<'a>) -> Self {
        Self {
            config: config.config,
            strict: config.strict,
            ..Default::default()
        }
    }

    fn report<'s>(&self, h: &'s Handler, span: Span, msg: &'s str) -> DiagnosticBuilder<'s> {
        if self.strict {
            h.struct_span_err(span, msg)
        } else {
            h.struct_span_warn(span, msg)
        }
    }
}

/// The transform visitor for stailwc.
///
/// This visitor is responsible for transforming the AST,
/// discovering and parsing tailwind directives, and converting
/// component declarations into equivalent styled components.
impl<'a> VisitMut for TransformVisitor<'a> {
    /// Handle jsx attributes and convert them into emotion.
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
            Some(JSXAttrValue::Lit(Lit::Str(Str{span, value, ..})))
            // tw={"h-4"}
            | Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Lit(Lit::Str(Str{span, value, ..}))),
                ..
            })) => {
                let d = match Directive::parse(LocatedSpan::new_extra(value, *span)) {
                    Ok((_, d)) => d,
                    Err(e) => {
                        HANDLER.with(|h| {
                            self.report(h,  *span, &e.to_string())
                                .note("unknown plugin")
                                .emit()
                        });
                        return;
                    },
                };

                let x = match d.to_literal(*span, &self.config) {
                    Ok(x) => x,
                    Err(e) => {
                        HANDLER.with(|h| {
                            self.report(h,  *span, &e.to_string())
                                .note("when evaluating plugin")
                                .emit()
                        });
                        return;
                    },
                };

                if let Some((span, _val)) = self.tw_attr_stack.push((*span, x)) {
                    HANDLER.with(|h| {
                        self.report(h, n.span, "tw attribute already exists, ignoring")
                            .span_note(
                                span,
                                "previous encountered here",
                            )
                            .emit()
                    });
                }
            }
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(box Expr::Ident(Ident { .. })),
                span,
                ..
            })) => {
                HANDLER.with(|h| {
                    self.report(h, *span, "variables are not supported")
                            .emit()
                });
            }
            _ => HANDLER.with(|h| {
                h.span_bug_no_panic(n.span, "unknown tw attribute, please file an issue")
            }),
        }
    }

    /// Visit all jsx elements to either:
    ///
    /// a) detect the TailwindStyle import and transform it into a css declaration
    /// b) extract any tw attributes and transform them into a css declarations
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if self.tw_style_imported && let JSXElementName::Ident(i) = &n.name && i.sym.eq("TailwindStyle") {

            let atom: Atom = css::format_css(
                true,
                self.config.theme.font_family.get("sans").map(|v| v.as_slice()).unwrap_or(&[]),
                self.config.theme.font_family.get("mono").map(|v| v.as_slice()).unwrap_or(&[])
            ).into();

            n.name = JSXElementName::Ident(Ident::new("Global".into(), i.span));
            n.attrs.push(JSXAttrOrSpread::JSXAttr(
                JSXAttr {
                    span: n.span,
                    name: JSXAttrName::Ident(Ident::new("styles".into(), n.span)),
                    value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        span: n.span,
                        expr: JSXExpr::Expr(Box::new(Expr::TaggedTpl(
                            TaggedTpl {
                                span: n.span,
                                tag: Box::new(Expr::Ident(Ident {
                                    span: n.span,
                                    sym: "css".into(), 
                                    optional: false
                                })),
                                type_params: None, tpl: Tpl{
                                    span: n.span,
                                    exprs: vec![],
                                    quasis: vec![TplElement{
                                        cooked: Some(atom.clone()),
                                        raw: atom,
                                        span: n.span,
                                        tail: true
                                    }],
                                }
                            })

                        ))
                    })),
                }
            ));
        }

        self.tw_attr_stack.inc_depth();
        n.attrs.visit_mut_children_with(self);

        let lit = match self.tw_attr_stack.pop() {
            Some((_, v)) => v,
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

        self.tw_attr_stack.dec_depth();
    }

    /// On discovery of a template tag, if it is a tailwind template tag,
    /// convert it to an emotion object.
    #[tracing::instrument]
    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        let extract_literal = || {
            let (text, span) = match n.tpl.quasis.as_slice() {
                [TplElement { raw, span, .. }] => (raw, span),
                _ => {
                    HANDLER.with(|h| {
                        h.span_err(n.span, "variables inside template tags are not supported")
                    });
                    return None;
                }
            };

            let d = match Directive::parse(LocatedSpan::new_extra(text, *span)) {
                Ok((_, d)) => d,
                Err(e) => {
                    HANDLER.with(|h| {
                        self.report(h, *span, "invalid syntax")
                            .note(&e.to_string())
                            .emit()
                    });
                    return None;
                }
            };

            match d.to_literal(*span, &self.config) {
                Ok(lit) => Some(lit),
                Err(e) => {
                    HANDLER.with(|h| {
                        self.report(h, *span, &e.to_string())
                            .note("when evaluating plugin")
                            .emit()
                    });
                    None
                }
            }
        };

        let transform = match &n.tag {
            box Expr::Ident(Ident { sym, .. }) if sym == "tw" => {
                if let Some(lit) = extract_literal() {
                    TplTransform::Style(lit)
                } else {
                    return;
                }
            }
            box Expr::Member(MemberExpr {
                obj: box Expr::Ident(Ident { sym, .. }),
                prop: MemberProp::Ident(ident),
                ..
            }) if sym == "tw" => {
                if let Some(lit) = extract_literal() {
                    TplTransform::Component(ident.to_owned(), lit)
                } else {
                    return;
                }
            }
            box Expr::Call(CallExpr {
                callee: Callee::Expr(box Expr::Ident(Ident { sym, .. })),
                args,
                ..
            }) if sym == "tw" => {
                if let Some(lit) = extract_literal() {
                    TplTransform::ComponentCustom(args.to_owned(), lit)
                } else {
                    return;
                }
            }
            _ => {
                n.visit_mut_children_with(self);
                return;
            }
        };

        if self.tw_tpl.replace(transform).is_some() {
            HANDLER.with(|h| {
                h.span_bug_no_panic(
                    n.span,
                    "encountered bad state in parsing, please file an issue",
                )
            });
        }
    }

    /// Visit an expression, optionally substituting the template tag with the
    /// generated emotion object.
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);
        match self.tw_tpl.take() {
            Some(TplTransform::Style(lit)) => {
                *n = Expr::Object(lit);
            }
            Some(TplTransform::Component(ident, lit)) => {
                *n = Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "_styled".into(),
                            optional: false,
                        })),
                        prop: MemberProp::Ident(ident),
                    }))),
                    args: vec![ExprOrSpread {
                        expr: Box::new(Expr::Object(lit)),
                        spread: None,
                    }],
                    type_args: None,
                });
            }
            Some(TplTransform::ComponentCustom(args, lit)) => {
                *n = Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Call(CallExpr {
                        args,
                        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "_styled".into(),
                            optional: false,
                        }))),
                        span: DUMMY_SP,
                        type_args: None,
                    }))),
                    type_args: None,
                    args: vec![ExprOrSpread {
                        expr: Box::new(Expr::Object(lit)),
                        spread: None,
                    }],
                })
            }
            None => {}
        }
    }

    /// Visit the import declarations, and mark whether tw_style is imported.
    fn visit_mut_module(&mut self, n: &mut Module) {
        n.body.drain_filter(|stmt| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl { src, .. })) = stmt {
                if src.value.eq("stailwc") {
                    self.tw_style_imported = true;
                    return true;
                }
            }

            false
        });
        n.visit_mut_children_with(self);
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
        Ok(config) => program.fold_with(&mut as_folder(TransformVisitor::new(config))),
        Err(error) => {
            HANDLER.with(|h| {
                h.struct_fatal("unable to parse tailwind config, aborting")
                    .note(&error.to_string())
                    .emit()
            });
            program
        }
    }
}
