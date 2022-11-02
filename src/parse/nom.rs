use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::{eof, opt, verify};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;
use swc_core::common::{BytePos, Span};
use tailwind_parse::Plugin;

type NomSpan<'a> = LocatedSpan<&'a str, Span>;

#[derive(Debug, PartialEq)]
pub struct Directive<'a> {
    pub exps: Vec<Expression<'a>>,
}

impl<'a> Directive<'a> {
    /**
     * Same as parse, but with an added check for an EOF.
     */
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        terminated(many0(Expression::parse).and(space0), eof)
            .map(|(exps, _)| Directive { exps })
            .parse(s)
    }

    fn parse_inner(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        many1(Expression::parse)
            .and(space0)
            .map(|(exps, _)| Directive { exps })
            .parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub negative: bool,
    pub modifiers: Vec<&'a str>,
    pub subject: Subject<'a>,
    pub alpha: Option<&'a str>,
    pub important: bool,
    pub span: Option<Span>,
}

impl<'a> Expression<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let mut negative = opt(char('-')).map(|o| o.is_some());
        let mut important = opt(char('!')).map(|o| o.is_some());

        let fst = take_while(|c| is_alphabetic(c as u8) || c == '-');
        let snd = |s: &NomSpan<'a>| {
            !s.is_empty() && is_alphabetic(s.chars().next().expect("not empty") as u8)
        };

        let verify = verify(fst, snd);

        let mut mods = many0(terminated(verify, char(':')));
        let mut subject = Subject::parse;

        let (s_next, _) = space0(s)?;
        let (s_next, mods) = mods.parse(s_next)?;
        let (s_next, negative) = negative.parse(s_next)?;
        let (s_next, subject) = subject.parse(s_next)?;
        let (s_next, important) = important.parse(s_next)?;

        let lo = s.extra.lo() + BytePos(s.location_offset() as u32 + 2);
        let hi = s_next.extra.lo() + BytePos(s_next.location_offset() as u32 + 1);

        Ok((
            s_next,
            Expression {
                alpha: None,
                important,
                modifiers: mods.iter().map(|&s| *s).collect(),
                negative,
                subject,
                span: Some(s.extra.with_lo(lo).with_hi(hi)),
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum Subject<'a> {
    Literal(Literal<'a>),
    Group(Directive<'a>),
}

/// The core 'rule' of a tailwind directive.
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

impl<'a> Subject<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let plugin = Plugin::parse;
        let value = verify(
            take_while(|c| is_alphanumeric(c as u8) || c == '-' || c == '.' || c == '/'),
            |s: &NomSpan<'a>| (*s).len() > 0,
        )
        .map(|val: NomSpan<'a>| (val, SubjectValue::Value(*val)));
        let css = delimited(char('['), take_while(|c| c != ']'), char(']'))
            .map(|css: NomSpan<'a>| (css, SubjectValue::Css(*css)));

        let subject_value = opt(alt((value, css)));

        let group = delimited(char('('), Directive::parse_inner, char(')')).map(Subject::Group);
        let literal = terminated(plugin, tag("-"))
            .and(subject_value)
            .map(|(cmd, value)| {
                let (span, value) = value.unzip();
                Subject::Literal(Literal {
                    cmd,
                    value,
                    span: Some(
                        s.extra
                            .with_lo(s.extra.lo() + BytePos(s.location_offset() as u32))
                            .with_hi(
                                s.extra.lo()
                                    + BytePos(
                                        span.map(|s| s.location_offset() as u32).unwrap_or(0),
                                    ),
                            ),
                    ),
                    full: &s[..],
                })
            });
        alt((literal, group))(s)
    }
}

#[cfg(test)]
mod test {

    use super::{Directive, Subject, SubjectValue};

    use nom_locate::LocatedSpan;
    use swc_core::common::DUMMY_SP;
    use tailwind_parse::{Border, Plugin, Position};
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

    #[test_case("relative", Plugin::Position(Position::Relative), None ; "when a subject has no value")]
    #[test_case("pl-3.5", Plugin::Pl, Some(SubjectValue::Value("3.5")) ; "when a subject has a . in it")]
    #[test_case("border-b-4", Plugin::Border(Some(Border::B)), Some(SubjectValue::Value("4")) ; "dash in plugin")]
    #[test_case("w-3/4", Plugin::W, Some(SubjectValue::Value("3/4")) ; "when a statement has /")]
    #[test_case("border-[10px]", Plugin::Border(None), Some(SubjectValue::Css("10px")) ; "arbitrary css")]
    #[test_case("border-[repeat(6,1fr)]", Plugin::Border(None), Some(SubjectValue::Css("repeat(6,1fr)")) ; "when braces are in arbitrary css")]
    #[test_case("border-[min-content min-content]", Plugin::Border(None), Some(SubjectValue::Css("min-content min-content")) ; "when spaces are in arbitrary css")]
    fn plugin_tests(s: &str, p: Plugin, v: Option<SubjectValue>) {
        let (_, s) = Subject::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        let lit = match s {
            Subject::Literal(l) => l,
            _ => panic!("should be a group"),
        };
        assert_eq!(lit.cmd, p, "correct plugin");
        assert_eq!(lit.value, v, "correct value");
    }

    #[test_case(" text-lg b-4  p-4 " ; "when a statement has irregular gaps")]
    #[test_case("dash-modifier:p-4" ; "when a modifier has a dash in it")]
    #[test_case("mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8" ; "random prefixes")]
    fn parse_tests(s: &str) {
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
