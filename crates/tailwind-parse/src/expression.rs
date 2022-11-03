use nom::{
    bytes::complete::take_while,
    character::{
        complete::{char, space0},
        is_alphabetic,
    },
    combinator::{opt, verify},
    multi::many0,
    sequence::terminated,
    IResult, Parser,
};
use swc_core::common::{BytePos, Span};

use crate::{subject::Subject, NomSpan};

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
