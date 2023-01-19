use nom::{character::complete::space0, combinator::eof, multi::many1, IResult, Parser};
use nom_locate::LocatedSpan;
use stailwc_swc_utils::merge_literals;
use swc_core::{
    common::{Span, DUMMY_SP},
    ecma::ast::ObjectLit,
};
use tailwind_config::TailwindConfig;

use crate::{Expression, ExpressionConversionError, NomSpan};

/// A directive is a list of expressions separated by one or more spaces.
/// An example is: `bg-red-500 text-white`
#[derive(Debug, PartialEq)]
pub struct Directive<'a> {
    pub exps: Vec<Expression<'a>>,
}

impl<'a> Directive<'a> {
    /// Same as parse, but with an added check for an EOF.     
    pub fn parse(s: NomSpan<'a>) -> (NomSpan<'a>, Self, Vec<LocatedSpan<&str, Span>>) {
        let mut exps = vec![];
        let mut errs = vec![];

        let (mut s, _) = space0::<LocatedSpan<&str, Span>, ()>.parse(s).unwrap();

        let mut exp = Expression::parse.and(space0);
        let mut fast_forward =
            nom::bytes::complete::take_while::<_, LocatedSpan<&str, Span>, ()>(|c: char| c != ' ')
                .and(space0);

        // todo: we can probably move the expression parser first
        loop {
            if let Ok((s_next, _)) = eof::<_, ()>.parse(s) {
                s = s_next;
                break;
            }

            let _parse_err = match exp.parse(s) {
                Ok((s_next, (exp, _))) => {
                    s = s_next;
                    exps.push(exp);
                    continue;
                }
                Err(e) => e,
            };

            let (s_next, (mut ffd, _)) = fast_forward
                .parse(s)
                .expect("fast-forward will always succeed");

            ffd.extra = ffd.extra.from_inner_byte_pos(
                ffd.location_offset() + 1,
                ffd.location_offset() + ffd.len() + 1,
            );

            errs.push(ffd);
            s = s_next;
        }

        (s, Directive { exps }, errs)
    }

    pub(crate) fn parse_inner(
        s: NomSpan<'a>,
    ) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        many1(Expression::parse)
            .and(space0)
            .map(|(exps, _)| Directive { exps })
            .parse(s)
    }

    /// Attempts to parse a literal from the given directive,
    pub fn to_literal(
        self,
        config: &'a TailwindConfig,
    ) -> (ObjectLit, Vec<ExpressionConversionError<'a>>) {
        self.exps
            .into_iter()
            .map(|e| {
                let span = e.span;
                e.to_literal(span.unwrap_or(DUMMY_SP), config)
            })
            .fold(
                (
                    ObjectLit {
                        span: DUMMY_SP,
                        props: vec![],
                    },
                    vec![],
                ),
                |(lit, mut errs), next| match next {
                    Ok(obj) => {
                        let obj = merge_literals(lit, obj);
                        (obj, errs)
                    }
                    Err(err) => {
                        errs.push(err);
                        (lit, errs)
                    }
                },
            )
    }
}
