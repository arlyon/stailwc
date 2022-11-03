use swc_core::ecma::ast::ObjectLit;
use tailwind_config::TailwindTheme;

use crate::{Not, Prose, SubjectValue};

pub fn prose(p: Prose, _rest: &Option<SubjectValue>, _theme: &TailwindTheme) -> Option<ObjectLit> {}

pub fn not(n: Not, _rest: &Option<SubjectValue>, _theme: &TailwindTheme) -> Option<ObjectLit> {}
