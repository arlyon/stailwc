#![feature(let_chains)]
#![feature(box_patterns)]
#![feature(iterator_try_reduce)]
#![feature(assert_matches)]

mod directive;
mod expression;
mod literal;
mod plugin;
mod plugin_impl;
mod subject;

pub type NomSpan<'a> = LocatedSpan<&'a str, Span>;

use nom_locate::LocatedSpan;
use swc_core::common::Span;

pub use directive::*;
pub use expression::*;
pub use literal::*;
pub use plugin::*;
pub use subject::*;

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use crate::{
        Border, Directive, Expression, Literal, Max, Plugin, Position, Subject, SubjectValue,
    };

    use nom_locate::LocatedSpan;
    use swc_core::common::DUMMY_SP;
    use test_case::test_case;

    #[test]
    fn directive() -> anyhow::Result<()> {
        let (rest, d) = Directive::parse(LocatedSpan::new_extra(
            "-h-4 md:bg-blue text-white! hover:(text-blue bg-white lg:text-black!)",
            DUMMY_SP,
        ))?;

        assert!(rest.len() == 0);

        Ok(())
    }

    #[test_case("flex!", None, None, true ; "important")]
    #[test_case("underline!", None, None, true ; "important with transparent command")]
    #[test_case("min-w-4!", Some(SubjectValue::Value("4")), None, true ; "important with rootless command")]
    #[test_case("text-blue-500/40", Some(SubjectValue::Value("blue-500")), Some(SubjectValue::Value("40")), false ; "handles transparent")]
    #[test_case("text-white/40!", Some(SubjectValue::Value("white")), Some(SubjectValue::Value("40")), true ; "handles transparent and important")]
    fn expression(
        s: &str,
        value_exp: Option<SubjectValue>,
        transparency: Option<SubjectValue>,
        important: bool,
    ) {
        let (rest, d) = Expression::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();

        if let Subject::Literal(Literal { value, .. }) = d.subject {
            assert_eq!(value, value_exp);
        }

        assert_eq!(d.important, important);
        assert_eq!(d.alpha, transparency);
        assert_matches!(*rest, "");
    }

    #[test_case("relative", Plugin::Position(Position::Relative), None ; "when a subject has no value")]
    #[test_case("pl-3.5", Plugin::Pl, Some(SubjectValue::Value("3.5")) ; "when a subject has a dot in it")]
    #[test_case("text-red-500", Plugin::Text, Some(SubjectValue::Value("red-500")) ; "when a subject has a dash")]
    #[test_case("border-b-4", Plugin::Border(Some(Border::B)), Some(SubjectValue::Value("4")) ; "dash in plugin")]
    #[test_case("border-4", Plugin::Border(None), Some(SubjectValue::Value("4")) ; "empty plugin subcommand")]
    #[test_case("max-w-4", Plugin::Max(Max::W), Some(SubjectValue::Value("4")) ; "rootless subcommand")]
    #[test_case("w-3/4", Plugin::W, Some(SubjectValue::Value("3/4")) ; "when a subject has a forward slash")]
    #[test_case("border-[10px]", Plugin::Border(None), Some(SubjectValue::Css("10px")) ; "arbitrary css")]
    #[test_case("border-[repeat(6,1fr)]", Plugin::Border(None), Some(SubjectValue::Css("repeat(6,1fr)")) ; "when braces are in arbitrary css")]
    #[test_case("border-[min-content min-content]", Plugin::Border(None), Some(SubjectValue::Css("min-content min-content")) ; "when spaces are in arbitrary css")]
    fn plugin(s: &str, p: Plugin, v: Option<SubjectValue>) {
        let (rest, s) = Subject::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        let lit = match s {
            Subject::Literal(l) => l,
            _ => panic!("should be a group"),
        };
        assert_eq!(lit.cmd, p, "correct plugin");
        assert_eq!(lit.value, v, "correct value");
        assert_matches!(*rest, "");
    }

    #[test_case("text-lg p-4" ; "basic")]
    #[test_case("border-b-4 p-4" ; "with subcommand")]
    #[test_case("   p-4    border-4" ; "when a statement has irregular gaps")]
    #[test_case("dash-modifier:p-4" ; "when a modifier has a dash in it")]
    #[test_case("mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8" ; "random prefixes")]
    #[test_case("relative rounded-2xl px-6 py-10 bg-primary-500 overflow-hidden shadow-xl sm:px-12 sm:py-20"; "example")]
    #[test_case("text-white/40 bg-white/50" ; "chained transparency")]
    fn directive_tests(s: &str) {
        Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
    }

    #[should_panic]
    #[test_case("-mod:sub" ; "when the minus is in the wrong place")]
    #[test_case("m0d:p-4" ; "when modifier has a number")]
    #[test_case("()" ; "rejects empty group")]
    fn parse_failure_tests(s: &str) {
        let (rest, d) = Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        assert_matches!(*rest, "");
    }

    #[test_case("40" ; "a number")]
    #[test_case("blue-500" ; "a color")]
    #[test_case("[10px]" ; "csss")]
    fn subject_value(s: &str) {
        let (rest, s) = SubjectValue::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        assert_matches!(*rest, "");
    }
}
