#![feature(box_patterns)]
#![feature(let_chains)]
#![feature(drain_filter)]
#![deny(clippy::unwrap_used)]
#![feature(once_cell)]
// bug in swc
#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod css;
mod depth_stack;
mod engine;
#[cfg(test)]
mod snapshot;
#[cfg(test)]
mod test;

use depth_stack::DepthStack;

use nom_locate::LocatedSpan;
use serde::Deserialize;
use swc_core::{
    common::{
        errors::{DiagnosticBuilder, Handler},
        util::take::Take,
        Span, DUMMY_SP,
    },
    ecma::{
        ast::{
            ArrayLit, BindingIdent, CallExpr, Callee, Decl, Expr, ExprOrSpread, Ident, ImportDecl,
            ImportNamedSpecifier, ImportSpecifier, JSXAttr, JSXAttrName, JSXAttrOrSpread,
            JSXAttrValue, JSXElementName, JSXExpr, JSXExprContainer, JSXOpeningElement, Lit,
            MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem, ObjectLit, Pat, Program, Stmt,
            Str, TaggedTpl, Tpl, TplElement, VarDecl, VarDeclKind, VarDeclarator,
        },
        atoms::Atom,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use tailwind_config::TailwindConfig;
use tailwind_parse::{
    Directive, ExpressionConversionError, LiteralConversionError, SubjectConversionError,
};

#[cfg(not(target_arch = "wasm32"))]
use swc_core::common::errors::HANDLER;
#[cfg(target_arch = "wasm32")]
use swc_core::plugin::errors::HANDLER;

#[derive(serde::Deserialize, Debug, Default)]
pub struct AppConfig<'a> {
    #[serde(borrow)]
    pub config: TailwindConfig<'a>,

    /// Strict mode throws an error when an unknown class is used.
    #[serde(default)]
    pub strict: bool,

    pub engine: Engine,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Engine {
    Emotion,
    StyledComponents,
}

impl Default for Engine {
    fn default() -> Self {
        Self::Emotion
    }
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

#[derive(Debug)]
pub struct TransformVisitor<'a> {
    config: &'a TailwindConfig<'a>,
    strict: bool,
    engine: Engine,
    /// This is treated as a stack because tw attrs can be nested
    tw_attr_stack: DepthStack<(Span, ObjectLit)>,
    tw_tpl: Option<TplTransform>,
    tw_style_imported: bool,
}

impl<'a> TransformVisitor<'a> {
    pub fn new(config: &'a AppConfig<'a>) -> Self {
        Self {
            config: &config.config,
            strict: config.strict,
            engine: config.engine,
            tw_attr_stack: Default::default(),
            tw_tpl: None,
            tw_style_imported: false,
        }
    }

    fn report<'s>(
        &self,
        h: &'s Handler,
        span: Span,
        msg: &'s str,
        suggestions: Option<&[&str]>,
    ) -> DiagnosticBuilder<'s> {
        let mut b = if self.strict {
            h.struct_span_err(span, msg)
        } else {
            h.struct_span_warn(span, msg)
        };

        if let Some(s) = suggestions && !s.is_empty() {
            b.allow_suggestions(true)
                .help(&format!("maybe you meant {s:?}"))
                .span_suggestions(
                    span,
                    "maybe you meant",
                    s.iter().map(|s| s.to_string()).collect(),
                );
        }

        b
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
                let (_s, d, errs)  = Directive::parse(LocatedSpan::new_extra(value, *span));

                for err in errs {
                    HANDLER.with(|h| {
                        self.report(h,  err.extra, "unknown plugin", None)
                            .emit()
                    });
                }

                let (x, errs) = d.to_literal(self.config);

                for e in errs {
                    HANDLER.with(|h| {
                        match e {
                            ExpressionConversionError::UnknownSubject(SubjectConversionError::InvalidLiteral(LiteralConversionError::InvalidArguments(_p, v, s)), span) => {
                                self
                                    .report(h, span, &format!("unknown parameter {}", v.as_str()), Some(&s))
                                    .emit()
                            }
                            _ => {
                                self.report(h,  *span, &e.to_string(), None)
                                    .emit()
                            }
                        }

                    });
                }

                if let Some((span, _val)) = self.tw_attr_stack.push((*span, x)) {
                    HANDLER.with(|h| {
                        self.report(h, n.span, "tw attribute already exists, ignoring", None)
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
                    self.report(h, *span, "variables are not supported", None)
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
            match self.engine {
                Engine::Emotion => {
                    let atom: Atom = css::format_css(
                        true,
                        self.config.theme.font_family.get("sans").map(|v| v.as_slice()).unwrap_or(&[]),
                        self.config.theme.font_family.get("mono").map(|v| v.as_slice()).unwrap_or(&[])
                    ).into();
                    self.emotion_global(i.span, n, atom)
                },
                Engine::StyledComponents => self.styled_components_global(i.span, n),
            }
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
                    return ObjectLit::dummy();
                }
            };

            let (_s, d, errs) = Directive::parse(LocatedSpan::new_extra(text, *span));

            for err in errs {
                HANDLER.with(|h| self.report(h, err.extra, "unknown plugin", None).emit());
            }

            let (lit, errs) = d.to_literal(self.config);

            for e in errs {
                HANDLER.with(|h| match e {
                    ExpressionConversionError::UnknownSubject(
                        SubjectConversionError::InvalidLiteral(
                            LiteralConversionError::InvalidArguments(_p, v, s),
                        ),
                        span,
                    ) => self
                        .report(
                            h,
                            span,
                            &format!("unknown parameter {}", v.as_str()),
                            Some(&s),
                        )
                        .emit(),
                    _ => self.report(h, *span, &e.to_string(), None).emit(),
                });
            }

            lit
        };

        let transform = match &n.tag {
            // tw`...`
            box Expr::Ident(Ident { sym, .. }) if sym == "tw" => {
                TplTransform::Style(extract_literal())
            }
            // tw.button`...`
            box Expr::Member(MemberExpr {
                obj: box Expr::Ident(Ident { sym, .. }),
                prop: MemberProp::Ident(ident),
                ..
            }) if sym == "tw" => TplTransform::Component(ident.to_owned(), extract_literal()),
            // tw(Button)`...`
            box Expr::Call(CallExpr {
                callee: Callee::Expr(box Expr::Ident(Ident { sym, .. })),
                args,
                ..
            }) if sym == "tw" => TplTransform::ComponentCustom(args.to_owned(), extract_literal()),
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
        for stmt in &mut n.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                src, specifiers, ..
            })) = stmt
            {
                let has_style_import = specifiers.iter().any(|s| match s {
                    ImportSpecifier::Named(ImportNamedSpecifier { local, .. }) => {
                        local.sym.eq("TailwindStyle")
                    }
                    _ => false,
                });

                if !src.value.eq("stailwc") || !has_style_import {
                    continue;
                }

                self.tw_style_imported = true;

                match self.engine {
                    Engine::Emotion => {
                        *stmt = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                            src: Box::new(Str {
                                raw: None,
                                value: "@emotion/react".into(),
                                span: DUMMY_SP,
                            }),
                            span: DUMMY_SP,
                            specifiers: vec![
                                ImportSpecifier::Named(ImportNamedSpecifier {
                                    span: DUMMY_SP,
                                    local: Ident::new("css".into(), DUMMY_SP),
                                    is_type_only: false,
                                    imported: None,
                                }),
                                ImportSpecifier::Named(ImportNamedSpecifier {
                                    span: DUMMY_SP,
                                    local: Ident::new("Global".into(), DUMMY_SP),
                                    is_type_only: false,
                                    imported: None,
                                }),
                            ],
                            asserts: None,
                            type_only: false,
                        }));
                    }
                    Engine::StyledComponents => {
                        // import createGlobalStyle and create the Global object
                        *stmt = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                            src: Box::new(Str {
                                raw: None,
                                value: "styled-components".into(),
                                span: DUMMY_SP,
                            }),
                            span: DUMMY_SP,
                            specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                                span: DUMMY_SP,
                                local: Ident::new("createGlobalStyle".into(), DUMMY_SP),
                                is_type_only: false,
                                imported: None,
                            })],
                            asserts: None,
                            type_only: false,
                        }));
                    }
                }
            }
        }

        if self.engine == Engine::StyledComponents && self.tw_style_imported {
            let atom = css::format_css(
                true,
                self.config
                    .theme
                    .font_family
                    .get("sans")
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
                self.config
                    .theme
                    .font_family
                    .get("mono")
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]),
            )
            .into();

            n.body
                .push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: vec![VarDeclarator {
                        span: DUMMY_SP,
                        definite: true,
                        name: Pat::Ident(BindingIdent {
                            id: Ident::new("Global".into(), DUMMY_SP),
                            type_ann: None,
                        }),
                        init: Some(Box::new(Expr::Call(CallExpr {
                            span: DUMMY_SP,
                            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                                "createGlobalStyle".into(),
                                DUMMY_SP,
                            )))),
                            args: vec![ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Tpl(Tpl {
                                    span: DUMMY_SP,
                                    exprs: vec![],
                                    quasis: vec![TplElement {
                                        span: DUMMY_SP,
                                        tail: true,
                                        cooked: None,
                                        raw: atom,
                                    }],
                                })),
                            }],
                            type_args: None,
                        }))),
                    }],
                })))));
        };

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
        Ok(config) => program.fold_with(&mut as_folder(TransformVisitor::new(&config))),
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
