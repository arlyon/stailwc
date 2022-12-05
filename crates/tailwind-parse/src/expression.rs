use nom::{
    bytes::complete::take_while1,
    character::{
        complete::{char, space0},
        is_alphanumeric,
    },
    combinator::opt,
    multi::many0,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use swc_core::{
    common::{BytePos, Span, DUMMY_SP},
    ecma::ast::{Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};
use tailwind_config::{Screens, TailwindConfig};

use crate::{subject::Subject, NomSpan, SubjectConversionError, SubjectValue};

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub negative: bool,
    pub modifiers: Vec<&'a str>,
    pub subject: Subject<'a>,
    pub alpha: Option<SubjectValue<'a>>,
    pub important: bool,
    pub span: Option<Span>,
}

#[derive(thiserror::Error, Debug)]
pub enum ExpressionConversionError<'a> {
    #[error("{0}")]
    UnknownSubject(SubjectConversionError<'a>),
    #[error("unknown modifier `{0}`")]
    UnknownModifier(&'a str),
}

impl<'a> Expression<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let single_mod = take_while1(|c| is_alphanumeric(c as u8) || c == '-');

        let mods = many0(terminated(single_mod, char(':')));
        let negative = opt(char('-')).map(|o| o.is_some());
        let subject = Subject::parse;
        let alpha = opt(preceded(char('/'), SubjectValue::parse));
        let important = opt(char('!')).map(|o| o.is_some());

        let (s_next, (((((_, mods), negative), subject), alpha), important)) = space0
            .and(mods)
            .and(negative)
            .and(subject)
            .and(alpha)
            .and(important)
            .parse(s)?;

        let lo = s.extra.lo() + BytePos(s.location_offset() as u32 + 2);
        let hi = s_next.extra.lo() + BytePos(s_next.location_offset() as u32 + 1);

        Ok((
            s_next,
            Expression {
                alpha,
                important,
                modifiers: mods.into_iter().map(|s| *s).collect(),
                negative,
                subject,
                span: Some(s.extra.with_lo(lo).with_hi(hi)),
            },
        ))
    }

    pub fn to_literal(
        self,
        span: Span,
        config: &TailwindConfig,
    ) -> Result<ObjectLit, ExpressionConversionError<'a>> {
        let mut object: ObjectLit = self
            .subject
            .to_literal(span, config)
            .map_err(ExpressionConversionError::UnknownSubject)?;

        for prop in &mut object.props {
            if let PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                value: box Expr::Lit(Lit::Str(Str { value, .. })),
                ..
            })) = prop
            {
                *value = format!(
                    "{}{}{}",
                    if self.negative { "-" } else { "" },
                    value,
                    if self.important { " !important" } else { "" }
                )
                .into();
            }
        }

        for modifier in &self.modifiers {
            let value = match config.theme.screens.get(modifier) {
                Some(Screens::Min(v)) => format!("@media (min-width: {v})").into(),
                _ => match *modifier {
                    "sm" => "@media (min-width: 640px)",
                    "md" => "@media (min-width: 768px)",
                    "lg" => "@media (min-width: 1024px)",
                    "xl" => "@media (min-width: 1280px)",
                    "2xl" => "@media (min-width: 1536px)",
                    "before" => ":before",
                    "after" => ":after",
                    "hover" => ":hover",
                    "first-letter" => "::first-letter",
                    "marker" => "*::marker, ::marker",
                    "selection" => "*::selection, ::selection",
                    "file" => "::file-selector-button",
                    "first-line" => "::first-line",
                    "placeholder" => "::placeholder",
                    "backdrop" => "::backdrop",
                    "dark" => match config.dark_mode {
                        "media" => "@media (prefers-color-scheme: dark)",
                        "class" => ".dark",
                        _ => continue,
                    },
                    "light" => match config.dark_mode {
                        "media" => "@media (prefers-color-scheme: light)",
                        _ => continue,
                    },
                    "focus" => ":focus",
                    "focus-within" => ":focus-within",
                    "first" => ":first-child",
                    "last" => ":last-child",
                    "only" => ":only-child",
                    "odd" => ":nth-child(odd)",
                    "even" => ":nth-child(even)",
                    "first-of-type" => ":first-of-type",
                    "last-of-type" => ":last-of-type",
                    "only-of-type" => ":only-of-type",
                    "visited" => ":visited",
                    "target" => ":target",
                    "open" => "[open]",
                    "default" => ":default",
                    "checked" => ":checked",
                    "indeterminate" => ":indeterminate",
                    "placeholder-shown" => ":placeholder-shown",
                    "autofill" => ":autofill",
                    "optional" => ":optional",
                    "required" => ":required",
                    "valid" => ":valid",
                    "invalid" => ":invalid",
                    "in-range" => ":in-range",
                    "out-of-range" => ":out-of-range",
                    "read-only" => ":read-only",
                    "empty" => ":empty",
                    "enabled" => ":enabled",
                    "focus-visible" => ":focus-visible",
                    "active" => ":active",
                    "disabled" => ":disabled",
                    "all" => "*",
                    "all-child" => "> *",
                    "sibling" => "~ *",
                    "hocus" => ":hover, :focus",
                    "link" => ":link",
                    "read-write" => ":read-write",
                    "svg" => "svg",
                    "even-of-type" => ":nth-of-type(even)",
                    "odd-of-type" => ":nth-of-type(odd)",
                    "ltr" => "[dir='ltr'] &",
                    "rtl" => "[dir='rtl'] &",
                    "motion-safe" => "@media (prefers-reduced-motion: no-preference)",
                    "motion-reduce" => "@media (prefers-reduced-motion: reduce)",
                    "print" => "@media print",
                    "screen" => "@media screen",
                    "portrait" => "@media portrait",
                    "landscape" => "@media landscape",
                    "group-hover" => ".group:hover &",
                    "group-focus" => ".group:focus &",
                    x => return Err(ExpressionConversionError::UnknownModifier(x)),
                }
                .into(),
            };

            object = ObjectLit {
                span: DUMMY_SP,
                props: vec![Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value,
                    }),
                    value: Box::new(Expr::Object(object)),
                })
                .into()],
            }
        }

        Ok(object)
    }
}
