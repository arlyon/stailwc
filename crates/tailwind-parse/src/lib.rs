#![feature(let_chains)]
#![feature(box_patterns)]
#![feature(iterator_try_reduce)]
#![feature(assert_matches)]
#![feature(unzip_option)]

mod directive;
mod eval;
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
    use std::assert_matches::assert_matches;

    use crate::{
        Border, Css, Directive, Display, Expression, Literal, Max, Plugin, Position, Subject,
        SubjectValue, TextDecoration, Value,
    };

    use itertools::Itertools;
    use nom_locate::LocatedSpan;
    use stailwc_swc_utils::sort_recursive;
    use swc_core::common::DUMMY_SP;
    use tailwind_config::{LineHeightOpt, TailwindConfig};
    use test_case::test_case;

    use pretty_assertions::assert_eq;

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
    #[test_case("min-w-4!", Some(SubjectValue::Value(Value("4"))), None, true ; "important with rootless command")]
    #[test_case("text-blue-500/40", Some(SubjectValue::Value(Value("blue-500"))), Some(SubjectValue::Value(Value("40"))), false ; "handles transparent")]
    #[test_case("text-white/40!", Some(SubjectValue::Value(Value("white"))), Some(SubjectValue::Value(Value("40"))), true ; "handles transparent and important")]
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
    #[test_case("pl-3.5", Plugin::Pl, Some(SubjectValue::Value(Value("3.5"))) ; "when a subject has a dot in it")]
    #[test_case("text-red-500", Plugin::Text, Some(SubjectValue::Value(Value("red-500"))) ; "when a subject has a dash")]
    #[test_case("border-b-4", Plugin::Border(Some(Border::B)), Some(SubjectValue::Value(Value("4"))) ; "dash in plugin")]
    #[test_case("border-4", Plugin::Border(None), Some(SubjectValue::Value(Value("4"))) ; "empty plugin subcommand")]
    #[test_case("max-w-4", Plugin::Max(Max::W), Some(SubjectValue::Value(Value("4"))) ; "rootless subcommand")]
    #[test_case("w-3/4", Plugin::W, Some(SubjectValue::Value(Value("3/4"))) ; "when a subject has a forward slash")]
    #[test_case("border-[10px]", Plugin::Border(None), Some(SubjectValue::Css(Css("10px"))) ; "arbitrary css")]
    #[test_case("border-[repeat(6,1fr)]", Plugin::Border(None), Some(SubjectValue::Css(Css("repeat(6,1fr)"))) ; "when braces are in arbitrary css")]
    #[test_case("border-[min-content min-content]", Plugin::Border(None), Some(SubjectValue::Css(Css("min-content min-content"))) ; "when spaces are in arbitrary css")]
    #[test_case("line-through", Plugin::TextDecoration(TextDecoration::LineThrough), None ; "when we have a transparent plugin")]
    #[test_case("table-cell", Plugin::Display(Display::TableCell), None ; "do not eagerly parse")]
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

    #[test_case(&["bg-white", "text-black"] ; "basic case")]
    #[test_case(&["prose", "w-full"] ; "prose")]
    #[test_case(&["text-sm", "font-bold", "text-black", "p-4", "m-8"] ; "more complicated")]
    fn directive_stable(s: &[&str]) {
        let mut config = TailwindConfig::default();
        config.theme.colors.insert("white", "#fff");
        config.theme.colors.insert("black", "#000");
        config.theme.width.insert("full", "100%");
        config.theme.margin.insert("auto", "auto");
        config.theme.font_weight.insert("bold", "bold");
        config.theme.padding.insert("4", "1rem");
        config.theme.margin.insert("8", "2rem");
        config
            .theme
            .font_size
            .insert("sm", ("1em", LineHeightOpt::Str("0.875rem")));

        let inputs = s
            .into_iter()
            .permutations(s.len())
            .map(|v| v.iter().map(|s| *s).join(" "))
            .collect::<Vec<_>>();

        let lits = inputs
            .iter()
            .map(|s| {
                let (_, d) = Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
                let lit = d.to_literal(DUMMY_SP, &config).unwrap();
                (s, sort_recursive(lit))
            })
            .collect::<Vec<_>>();

        for items in lits.windows(2) {
            let (s1, item1) = &items[0];
            let (s2, item2) = &items[1];

            assert_eq!(item1, item2, "\n\n`{}` != `{}`", s1, s2);
        }
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
