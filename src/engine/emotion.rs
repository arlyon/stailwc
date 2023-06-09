use swc_core::{
    common::Span,
    ecma::{
        ast::{
            Expr, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName,
            JSXExpr, JSXExprContainer, JSXOpeningElement, TaggedTpl, Tpl, TplElement,
        },
        atoms::Atom,
    },
};

use crate::TransformVisitor;

impl<'a> TransformVisitor<'a> {
    pub fn emotion_global(
        &mut self,
        mut jsx_ident_span: Span,
        jsx_opening_element: &mut JSXOpeningElement,
        atom: Atom,
    ) {
        if let Some(import_span) = self.tw_style_imported {
            jsx_ident_span.ctxt = import_span.ctxt;
            jsx_opening_element.name =
                JSXElementName::Ident(Ident::new("Global".into(), jsx_ident_span));
            jsx_opening_element
                .attrs
                .push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                    span: jsx_opening_element.span,
                    name: JSXAttrName::Ident(Ident::new("styles".into(), jsx_opening_element.span)),
                    value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        span: jsx_opening_element.span,
                        expr: JSXExpr::Expr(Box::new(Expr::TaggedTpl(TaggedTpl {
                            span: jsx_opening_element.span,
                            tag: Box::new(Expr::Ident(Ident {
                                span: jsx_opening_element.span,
                                sym: "css".into(),
                                optional: false,
                            })),
                            type_params: None,
                            tpl: Box::new(Tpl {
                                span: jsx_opening_element.span,
                                exprs: vec![],
                                quasis: vec![TplElement {
                                    cooked: Some(atom.clone()),
                                    raw: atom,
                                    span: jsx_opening_element.span,
                                    tail: true,
                                }],
                            }),
                        }))),
                    })),
                }));
        }
    }
}
