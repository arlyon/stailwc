use swc_core::common::Span;

use crate::Plugin;

/// The core 'rule' of a tailwind directive.
///
/// Example: `text-2xl` is a `Plugin` with a value of `2xl`.
#[derive(Debug, PartialEq, Eq)]
pub struct Literal<'a> {
    pub cmd: Plugin,
    pub value: Option<SubjectValue<'a>>,
    pub span: Option<Span>,
    pub full: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubjectValue<'a> {
    Value(&'a str),
    Css(&'a str),
}

impl<'a> SubjectValue<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            SubjectValue::Value(s) => s,
            SubjectValue::Css(s) => s,
        }
    }
}
