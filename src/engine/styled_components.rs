use swc_core::{
    common::Span,
    ecma::ast::{Ident, JSXElementName, JSXOpeningElement},
};

use crate::TransformVisitor;

impl<'a> TransformVisitor<'a> {
    pub fn styled_components_global(
        &mut self,
        mut jsx_ident_span: Span,
        n: &mut JSXOpeningElement,
    ) {
        if let Some(span) = self.tw_style_imported {
            jsx_ident_span.ctxt = span.ctxt;
            n.name = JSXElementName::Ident(Ident::new("Global".into(), jsx_ident_span));
        }
    }
}
