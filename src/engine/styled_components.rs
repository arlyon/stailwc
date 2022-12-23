use swc_core::{
    common::Span,
    ecma::{
        ast::{Ident, JSXElementName, JSXOpeningElement},
        atoms::Atom,
    },
};

use crate::TransformVisitor;

impl<'a> TransformVisitor<'a> {
    pub fn styled_components_global(&mut self, span: Span, n: &mut JSXOpeningElement) {
        n.name = JSXElementName::Ident(Ident::new("Global".into(), span));
    }
}
