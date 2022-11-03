#![feature(let_chains)]

mod directive;
mod expression;
mod literal;
mod plugin;
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
    use crate::{Directive, Expression, Plugin, Subject, SubjectValue, Position, Border, Max};

    use nom_locate::LocatedSpan;
    use swc_core::common::DUMMY_SP;
    use test_case::test_case;

    #[test]
    fn directive() -> anyhow::Result<()> {
        let (rest, d) = Directive::parse(LocatedSpan::new_extra(
            "-h-4 md:bg-blue text-white! hover:(text-blue bg-white lg:text-black!)",
            DUMMY_SP,
        ))?;

        println!("{:#?}", d);

        assert!(rest.len() == 0);

        Ok(())
    }

    #[test_case("mx-auto!", true ; "important")]
    #[test_case("underline!", true ; "important with transparent command")]
    #[test_case("min-w-4!", true ; "important with rootless command")]
    fn expression(s: &str, important: bool) {
        let (_, d) = Expression::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        assert_eq!(d.important, important)
    }

    #[test_case("relative", Plugin::Position(Position::Relative), None ; "when a subject has no value")]
    #[test_case("pl-3.5", Plugin::Pl, Some(SubjectValue::Value("3.5")) ; "when a subject has a dot in it")]
    #[test_case("text-red-500", Plugin::Text, Some(SubjectValue::Value("red-500")) ; "when a subject has a dash in it")]
    #[test_case("border-b-4", Plugin::Border(Some(Border::B)), Some(SubjectValue::Value("4")) ; "dash in plugin")]
    #[test_case("border-4", Plugin::Border(None), Some(SubjectValue::Value("4")) ; "empty plugin subcommand")]
    #[test_case("max-w-4", Plugin::Max(Max::W), Some(SubjectValue::Value("4")) ; "rootless subcommand")]
    #[test_case("w-3/4", Plugin::W, Some(SubjectValue::Value("3/4")) ; "when a statement has /")]
    #[test_case("border-[10px]", Plugin::Border(None), Some(SubjectValue::Css("10px")) ; "arbitrary css")]
    #[test_case("border-[repeat(6,1fr)]", Plugin::Border(None), Some(SubjectValue::Css("repeat(6,1fr)")) ; "when braces are in arbitrary css")]
    #[test_case("border-[min-content min-content]", Plugin::Border(None), Some(SubjectValue::Css("min-content min-content")) ; "when spaces are in arbitrary css")]
    fn plugin(s: &str, p: Plugin, v: Option<SubjectValue>) {
        let (_, s) = Subject::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        let lit = match s {
            Subject::Literal(l) => l,
            _ => panic!("should be a group"),
        };
        assert_eq!(lit.cmd, p, "correct plugin");
        assert_eq!(lit.value, v, "correct value");
    }

    #[test_case("text-lg p-4" ; "basic")]
    #[test_case("border-b-4 p-4" ; "with subcommand")]
    #[test_case("   p-4    border-4" ; "when a statement has irregular gaps")]
    #[test_case("dash-modifier:p-4" ; "when a modifier has a dash in it")]
    #[test_case("mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8" ; "random prefixes")]
    fn directive_tests(s: &str) {
        Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
    }

    #[should_panic]
    #[test_case("-mod:sub" ; "when the minus is in the wrong place")]
    #[test_case("m0d:p-4" ; "when modifier has a number")]
    #[test_case("()" ; "rejects empty group")]
    fn parse_failure_tests(s: &str) {
        let (rest, d) = Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        println!("rest: {}, d: {:?}", rest, d);
    }
}
