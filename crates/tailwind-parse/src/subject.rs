use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::{opt, verify};
use nom::sequence::preceded;
use nom::IResult;
use nom::{branch::alt, bytes::complete::take_while, sequence::delimited, Parser};
use swc_core::common::BytePos;

use crate::{
    directive::Directive,
    literal::{Literal, SubjectValue},
};
use crate::{NomSpan, Plugin};

#[derive(Debug, PartialEq)]
pub enum Subject<'a> {
    Literal(Literal<'a>),
    Group(Directive<'a>),
}

impl<'a> Subject<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let plugin = Plugin::parse;
        let value = verify(
            take_while(|c| is_alphanumeric(c as u8) || c == '-' || c == '.' || c == '/'),
            |s: &NomSpan<'a>| (*s).len() > 0,
        )
        .map(|val: NomSpan<'a>| (val, SubjectValue::Value(&val)));
        let css = delimited(char('['), take_while(|c| c != ']'), char(']'))
            .map(|css: NomSpan<'a>| (css, SubjectValue::Css(&css)));

        let subject_value = opt(preceded(tag("-"), alt((value, css))));

        let group = delimited(char('('), Directive::parse_inner, char(')')).map(Subject::Group);
        let literal = plugin.and(subject_value).map(|(cmd, value)| {
            let (span, value) = value.unzip();
            Subject::Literal(Literal {
                cmd,
                value,
                span: Some(
                    s.extra
                        .with_lo(s.extra.lo() + BytePos(s.location_offset() as u32))
                        .with_hi(
                            s.extra.lo()
                                + BytePos(span.map(|s| s.location_offset() as u32).unwrap_or(0)),
                        ),
                ),
                full: &s[..],
            })
        });
        alt((literal, group))(s)
    }
}
