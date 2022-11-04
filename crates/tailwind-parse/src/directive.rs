use nom::{
    character::complete::space0,
    combinator::eof,
    multi::{many0, many1},
    sequence::terminated,
    IResult, Parser,
};
use stailwc_swc_utils::merge_literals;
use swc_core::{
    common::{Span, DUMMY_SP},
    ecma::ast::ObjectLit,
};
use tailwind_config::TailwindConfig;

use crate::{Expression, ExpressionConversionError, NomSpan};

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

    pub fn to_literal(
        self,
        span: Span,
        config: &TailwindConfig,
    ) -> Result<ObjectLit, ExpressionConversionError<'a>> {
        self.exps
            .into_iter()
            .map(|e| e.to_literal(span, config))
            .try_fold(
                ObjectLit {
                    span: DUMMY_SP,
                    props: vec![],
                },
                |curr, next| Ok(merge_literals(curr, next?)),
            )
    }
}
