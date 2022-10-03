use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::{char, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::{eof, opt, verify};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};
use swc_core::common::Span;

#[derive(Debug, PartialEq)]
pub struct Directive<'a> {
    pub exps: Vec<Expression<'a>>,
}

impl<'a> Directive<'a> {
    /**
     * Same as parse, but with an added check for an EOF.
     */
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        terminated(many0(Expression::parse).and(space0), eof)
            .map(|(exps, _)| Directive { exps })
            .parse(s)
    }

    fn parse_inner(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
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
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        let negative = opt(char('-')).map(|o| o.is_some());
        let important = opt(char('!')).map(|o| o.is_some());
        let mods = many0(terminated(
            verify(
                take_while(|c| is_alphabetic(c as u8) || c == '-'),
                |s: &str| {
                    !s.is_empty() && is_alphabetic(s.chars().next().expect("not empty") as u8)
                },
            ),
            char(':'),
        ));
        let subject = Subject::parse;

        space0
            .and(mods)
            .and(negative)
            .and(subject)
            .and(important)
            .map(
                |((((_, modifiers), negative), subject), important)| Expression {
                    alpha: None,
                    important,
                    modifiers,
                    negative,
                    subject,
                    span: None,
                },
            )
            .parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Subject<'a> {
    Literal(Literal<'a>),
    Group(Directive<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Literal<'a> {
    pub cmd: &'a str,
    pub value: Option<SubjectValue<'a>>,
    pub span: Option<Span>,
    pub full: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum SubjectValue<'a> {
    Value(&'a str),
    Css(&'a str),
}

impl<'a> Subject<'a> {
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        let literal = verify(
            take_while(|c| is_alphanumeric(c as u8) || ['-', '.', '/'].contains(&c)),
            |c: &str| !c.is_empty() && is_alphabetic(c.chars().next().expect("not empty") as u8),
        )
        .and(opt(delimited(
            char('['),
            take_while(|c| c != ']'),
            char(']'),
        )))
        .map(|(lit, css)| match css {
            Some(css) => Subject::Literal(Literal {
                cmd: lit,
                value: Some(SubjectValue::Css(css)),
                span: None,
                full: s,
            }),
            None => {
                let (a, b) = lit
                    .rfind('-')
                    .map(|idx| (&lit[0..idx], &lit[idx + 1..lit.len()]))
                    .map(|(a, b)| (a, Some(b)))
                    .unwrap_or((lit, None));
                Subject::Literal(Literal {
                    cmd: a,
                    value: b.map(|b| SubjectValue::Value(b)),
                    span: None,
                    full: s,
                })
            }
        });
        let group = delimited(char('('), Directive::parse_inner, char(')')).map(Subject::Group);
        alt((literal, group))(s)
    }
}

#[cfg(test)]
mod test {

    use super::Directive;

    use test_case::test_case;

    #[test]
    fn directive() -> anyhow::Result<()> {
        let (rest, d) = Directive::parse(
            "-h-4 md:bg-blue text-white! hover:(text-blue bg-white lg:text-black!)",
        )?;

        println!("{:#?}", d);

        assert!(rest.len() == 0);

        Ok(())
    }

    #[test_case(" should handle  spacing " ; "when a statement has irregular gaps")]
    #[test_case("dash-modifier:call" ; "when a modifier has a dash in it")]
    #[test_case("pl-3.5" ; "when a subject has a . in it")]
    #[test_case("grid-cols-[repeat(6,1fr)]" ; "when braces are in arbitrary css")]
    #[test_case("grid-cols-[min-content min-content]" ; "when spaces are in arbitrary css")]
    #[test_case("relative z-10 m-auto w-3/4" ; "when a statement as /")]
    #[test_case("mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8" ; "random prefixes")]
    fn parse_tests(s: &str) {
        Directive::parse(s).unwrap();
    }

    #[should_panic]
    #[test_case("-5" ; "when subject does not start with a letter")]
    #[test_case("-mod:sub" ; "when the minus is in the wrong place")]
    #[test_case("m0d:sub" ; "when modifier has a number")]
    #[test_case("()" ; "rejects empty group")]
    fn parse_failure_tests(s: &str) {
        let (rest, d) = Directive::parse(s).unwrap();
        println!("rest: {}, d: {:?}", rest, d);
    }
}
