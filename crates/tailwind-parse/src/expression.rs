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
use swc_core::{
    common::{BytePos, Span, DUMMY_SP},
    ecma::ast::{Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};
use tailwind_config::{Screens, TailwindConfig};

use crate::{subject::Subject, NomSpan, SubjectConversionError};

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub negative: bool,
    pub modifiers: Vec<&'a str>,
    pub subject: Subject<'a>,
    pub alpha: Option<&'a str>,
    pub important: bool,
    pub span: Option<Span>,
}

#[derive(thiserror::Error, Debug)]
pub enum ExpressionConversionError<'a> {
    #[error("unknown subject `{0}`")]
    UnknownSubject(SubjectConversionError<'a>),
    #[error("unknown modifier `{0}`")]
    UnknownModifier(&'a str),
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

    pub fn to_literal(
        self,
        span: Span,
        config: &TailwindConfig,
    ) -> Result<ObjectLit, ExpressionConversionError<'a>> {
        let mut object: ObjectLit = self
            .subject
            .to_literal(span, config)
            .map_err(|text| ExpressionConversionError::UnknownSubject(text))?;

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
