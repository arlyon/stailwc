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
    pub fn emotion_global(&mut self, span: Span, n: &mut JSXOpeningElement, atom: Atom) {
        n.name = JSXElementName::Ident(Ident::new("Global".into(), span));
        n.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: n.span,
            name: JSXAttrName::Ident(Ident::new("styles".into(), n.span)),
            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: n.span,
                expr: JSXExpr::Expr(Box::new(Expr::TaggedTpl(TaggedTpl {
                    span: n.span,
                    tag: Box::new(Expr::Ident(Ident {
                        span: n.span,
                        sym: "css".into(),
                        optional: false,
                    })),
                    type_params: None,
                    tpl: Tpl {
                        span: n.span,
                        exprs: vec![],
                        quasis: vec![TplElement {
                            cooked: Some(atom.clone()),
                            raw: atom,
                            span: n.span,
                            tail: true,
                        }],
                    },
                }))),
            })),
        }));
    }
}
