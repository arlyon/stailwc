use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::{char, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::{eof, opt, verify};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;
use swc_core::common::{BytePos, Span};

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
    pub cmd: &'a str,
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
        let x = take_while(|c| is_alphanumeric(c as u8) || ['-', '.', '/'].contains(&c));
        let y = |c: &NomSpan<'a>| {
            !c.is_empty() && is_alphabetic(c.chars().next().expect("not empty") as u8)
        };
        let literal = verify(x, y);

        let literal = literal
            .and(opt(delimited(
                char('['),
                take_while(|c| c != ']'),
                char(']'),
            )))
            .map(|(span, span2_opt): (NomSpan, _)| match span2_opt {
                Some(span2) => Subject::Literal(Literal {
                    cmd: &span,
                    value: Some(SubjectValue::Css(&span2)),
                    span: Some(
                        s.extra
                            .with_lo(s.extra.lo() + BytePos(s.location_offset() as u32 + 2))
                            .with_hi(s.extra.lo() + BytePos(span2.location_offset() as u32 + 1)),
                    ),
                    full: &s[..span2.len()],
                }),
                None => {
                    let (cmd, b) = span
                        .rfind('-')
                        .map(|idx| (&span[0..idx], &span[idx + 1..span.len()]))
                        .map(|(a, b)| (a, Some(b)))
                        .unwrap_or((*span, None));
                    Subject::Literal(Literal {
                        cmd,
                        value: b.map(SubjectValue::Value),
                        span: Some(
                            s.extra
                                .with_lo(s.extra.lo() + BytePos(s.location_offset() as u32 + 2))
                                .with_hi(s.extra.lo() + BytePos(span.location_offset() as u32 + 1)),
                        ),
                        full: &s[..span.len()],
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

    #[test_case(" should handle  spacing " ; "when a statement has irregular gaps")]
    #[test_case("dash-modifier:call" ; "when a modifier has a dash in it")]
    #[test_case("pl-3.5" ; "when a subject has a . in it")]
    #[test_case("grid-cols-[repeat(6,1fr)]" ; "when braces are in arbitrary css")]
    #[test_case("grid-cols-[min-content min-content]" ; "when spaces are in arbitrary css")]
    #[test_case("relative z-10 m-auto w-3/4" ; "when a statement as /")]
    #[test_case("mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8" ; "random prefixes")]
    fn parse_tests(s: &str) {
        Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
    }

    #[should_panic]
    #[test_case("-5" ; "when subject does not start with a letter")]
    #[test_case("-mod:sub" ; "when the minus is in the wrong place")]
    #[test_case("m0d:sub" ; "when modifier has a number")]
    #[test_case("()" ; "rejects empty group")]
    fn parse_failure_tests(s: &str) {
        let (rest, d) = Directive::parse(LocatedSpan::new_extra(s, DUMMY_SP)).unwrap();
        println!("rest: {}, d: {:?}", rest, d);
    }
}
