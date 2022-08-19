use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::{eof, opt, verify};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

#[derive(Debug, PartialEq)]
pub struct Directive<'a> {
    pub exps: Vec<Expression<'a>>,
}

impl<'a> Directive<'a> {
    /**
     * Same as parse, but with an added check for an EOF.
     */
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        terminated(separated_list0(char(' '), Expression::parse), eof)
            .map(|exps| Directive { exps })
            .parse(s)
    }

    fn parse_inner(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        separated_list1(char(' '), Expression::parse)
            .map(|exps| Directive { exps })
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
}

impl<'a> Expression<'a> {
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        let negative = opt(char('-')).map(|o| o.is_some());
        let important = opt(char('!')).map(|o| o.is_some());
        let mods = many0(terminated(
            take_while(|c| is_alphabetic(c as u8)),
            char(':'),
        ));
        let subject = Subject::parse;

        mods.and(negative)
            .and(subject)
            .and(important)
            .map(|(((modifiers, negative), subject), important)| Expression {
                alpha: None,
                important,
                modifiers,
                negative,
                subject,
            })
            .parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Subject<'a> {
    Literal(&'a str),
    Group(Directive<'a>),
}

impl<'a> Subject<'a> {
    pub fn parse(s: &'a str) -> IResult<&'a str, Self, nom::error::Error<&'a str>> {
        let literal = verify(
            take_while(|c| is_alphanumeric(c as u8) || ['-', '[', ']'].contains(&c)),
            |c: &str| !c.is_empty() && is_alphabetic(c.chars().next().unwrap() as u8),
        )
        .map(Subject::Literal);
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

    #[should_panic]
    #[test_case("-5" ; "when subject does not start with a letter")]
    #[test_case("-mod:sub" ; "when the minus is in the wrong place")]
    #[test_case("m0d:sub" ; "when modifier has a number")]
    #[test_case("()" ; "rejects empty group")]
    fn parse_failure_tests(s: &str) {
        Directive::parse(s).unwrap();
    }
}
