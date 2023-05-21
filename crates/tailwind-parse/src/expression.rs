use nom::{
    bytes::complete::take_while1,
    character::{
        complete::{char, multispace0},
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

use crate::{subject::Subject, NomSpan, SubjectConversionError, Value};

/// An expression is a single tailwind class. An example is: `bg-red-500`
///
/// It can have modifiers, a subject, an alpha value, and be important.
///
/// Examples:
/// - `bg-red-500`
/// - `bg-red-500/50`
/// - `bg-red-500/50!`
/// - `dark:hover:bg-red-800`
/// - -px-4
/// - dark:(bg-red-800 hover:bg-red-900)
#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub negative: bool,
    pub modifiers: Vec<&'a str>,
    pub subject: Subject<'a>,
    pub alpha: Option<Value<'a>>,
    pub important: bool,
    pub span: Option<Span>,
}

#[derive(thiserror::Error, Debug)]
pub enum ExpressionConversionError<'a> {
    #[error("{0}")]
    UnknownSubject(SubjectConversionError<'a>, Span),
    #[error("unknown modifier `{0}`")]
    UnknownModifier(&'a str, Span),
}

impl<'a> ExpressionConversionError<'a> {
    pub fn span(&self) -> Span {
        match self {
            ExpressionConversionError::UnknownSubject(_, span) => *span,
            ExpressionConversionError::UnknownModifier(_, span) => *span,
        }
    }
}

impl<'a> Expression<'a> {
    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
        let single_mod = take_while1(|c| is_alphanumeric(c as u8) || c == '-');

        let mods = many0(terminated(single_mod, char(':')));
        let negative = opt(char('-')).map(|o| o.is_some());
        let subject = Subject::parse;
        let alpha = opt(preceded(char('/'), Value::parser().map(|t| t.1)));
        let important = opt(char('!')).map(|o| o.is_some());

        let (s_next, (((((_, mods), negative), subject), alpha), important)) = multispace0
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
        config: &'a TailwindConfig,
    ) -> Result<ObjectLit, ExpressionConversionError<'a>> {
        let mut object: ObjectLit = self
            .subject
            .to_literal(span, config, self.alpha)
            .map_err(|e| ExpressionConversionError::UnknownSubject(e, span))?;

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
                    "group-target" => ".group:target &",
                    "group-visited" => ".group:visited &",
                    "group-not-link" => ".group:not(:link) &",
                    "group-not-odd-of-type" => ".group:not(:nth-of-type(odd)) &",
                    "group-not-even-of-type" => ".group:not(:nth-of-type(even)) &",
                    "group-not-disabled" => ".group:not(:disabled) &",
                    "group-not-read-write" => ".group:not(:read-write) &",
                    "group-not-only-of-type" => ".group:not(:only-of-type) &",
                    "peer-target" => ".peer:target ~ &",
                    "peer-not-svg" => ".peer:not(svg) ~ &",
                    "peer-not-odd-of-type" => ".peer:not(:nth-of-type(odd)) ~ &",
                    "peer-not-link" => ".peer:not(:link) ~ &",
                    "peer-not-sibling" => ".peer:not(~ *) ~ &",
                    "peer-not-even-of-type" => ".peer:not(:nth-of-type(even)) ~ &",
                    "peer-not-all-child" => ".peer:not(> *) ~ &",
                    "peer-not-enabled" => ".peer:not(:enabled) ~ &",
                    "peer-not-all" => ".peer:not(*) ~ &",
                    "peer-not-disabled" => ".peer:not(:disabled) ~ &",
                    "peer-not-read-write" => ".peer:not(:read-write) ~ &",
                    "peer-not-active" => ".peer:not(:active) ~ &",
                    "peer-only-of-type" => ".peer:only-of-type ~ &",
                    "peer-visited" => ".peer:visited ~ &",
                    "peer-not-focus-visited" => ".peer:not(:focus:visited) ~ &",
                    "peer-not-open" => ".peer:not([open]) ~ &",
                    "peer-not-target" => ".peer:not(:target) ~ &",
                    "peer-not-first" => ".peer:not(:first-child) ~ &",
                    "peer-first-of-type" => ".peer:first-of-type ~ &",
                    "peer-off-of-type" => ".peer:only-of-type ~ &",
                    "rtl" => "[dir='rtl'] &",
                    "motion-safe" => "@media (prefers-reduced-motion: no-preference)",
                    "motion-reduce" => "@media (prefers-reduced-motion: reduce)",
                    "print" => "@media print",
                    "screen" => "@media screen",
                    "portrait" => "@media portrait",
                    "landscape" => "@media landscape",
                    "group-hover" => ".group:hover &",
                    "group-focus" => ".group:focus &",
                    x => {
                        return Err(ExpressionConversionError::UnknownModifier(
                            x,
                            self.span.unwrap_or(DUMMY_SP),
                        ))
                    }
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
