use nom::{
    character::complete::space0,
    combinator::eof,
    multi::{many0, many1},
    sequence::terminated,
    IResult, Parser,
};

use crate::{Expression, NomSpan};

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

    pub(crate) fn parse_inner(
        s: NomSpan<'a>,
    ) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        many1(Expression::parse)
            .and(space0)
            .map(|(exps, _)| Directive { exps })
            .parse(s)
    }
}
