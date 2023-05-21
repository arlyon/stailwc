use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::{sequence::delimited, Parser};
use swc_core::common::{BytePos, Span};
use swc_core::ecma::ast::ObjectLit;
use tailwind_config::TailwindConfig;

use crate::{
    directive::Directive,
    literal::{Literal, SubjectValue},
};
use crate::{ExpressionConversionError, LiteralConversionError, NomSpan, Plugin, Value};

#[derive(Debug, PartialEq)]
pub enum Subject<'a> {
    Literal(Literal<'a>),
    Group(Directive<'a>),
}

#[derive(thiserror::Error, Debug)]
pub enum SubjectConversionError<'a> {
    /// note: boxed because parsing a group can cause an expression to be parsed
    #[error("{0}")]
    InvalidExpression(Box<ExpressionConversionError<'a>>),
    #[error("{0}")]
    InvalidLiteral(LiteralConversionError<'a>),
}

impl<'a> Subject<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let plugin = Plugin::parse;

        let subject_value = opt(preceded(char('-'), SubjectValue::parse_with_span));

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
            })
        });

        literal.or(group).parse(s)
    }

    pub fn to_literal(
        self,
        span: Span,
        config: &'a TailwindConfig,
        alpha: Option<Value<'a>>,
    ) -> Result<ObjectLit, SubjectConversionError<'a>> {
        match self {
            Subject::Literal(lit) => lit
                .to_object_lit(span, &config.theme, &alpha)
                .map_err(SubjectConversionError::InvalidLiteral),
            Subject::Group(dir) => {
                let (obj, mut errs) = dir.to_literal(config);

                if errs.is_empty() {
                    Ok(obj)
                } else {
                    Err(SubjectConversionError::InvalidExpression(Box::new(
                        errs.pop().unwrap(),
                    )))
                }
            }
        }
    }
}
