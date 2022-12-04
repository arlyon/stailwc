use std::fmt::Display;

use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::character::is_alphabetic;
use nom::character::is_digit;
use nom::combinator::verify;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::Err;
use nom::IResult;
use nom::Parser;
use nom::Slice;
use swc_core::{common::Span, ecma::ast::ObjectLit};
use tailwind_config::TailwindTheme;

use crate::eval::{plugin, prose};
use crate::NomSpan;
use crate::Plugin;

/// The core 'rule' of a tailwind directive.
///
/// Example: `text-2xl` is a `Plugin` with a value of `2xl`.
#[derive(Debug, PartialEq, Eq)]
pub struct Literal<'a> {
    pub cmd: Plugin,
    pub value: Option<SubjectValue<'a>>,
    pub span: Option<Span>,
}

#[derive(thiserror::Error, Debug)]
pub enum LiteralConversionError<'a> {
    #[error("missing argument for `{0:?}`")]
    MissingArguments(Plugin),
    #[error("invalid argument for `{0:?}` - `{1}`")]
    InvalidArguments(Plugin, SubjectValue<'a>),
}

impl<'a> LiteralConversionError<'a> {
    pub fn new<'b: 'a>(cmd: Plugin, value: Option<SubjectValue<'a>>) -> Self {
        match value {
            Some(value) => Self::InvalidArguments(cmd, value),
            None => Self::MissingArguments(cmd),
        }
    }
}

enum PluginType {
    Singular(fn() -> Option<ObjectLit>),
    Required(fn(&Value, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&Value>, &TailwindTheme) -> Option<ObjectLit>),
    RequiredArbitrary(fn(&SubjectValue, &TailwindTheme) -> Option<ObjectLit>),
    OptionalArbitrary(fn(Option<&SubjectValue>, &TailwindTheme) -> Option<ObjectLit>),
}

impl<'a> Literal<'a> {
    pub fn to_object_lit(
        self,
        _span: Span,
        theme: &TailwindTheme,
    ) -> Result<ObjectLit, LiteralConversionError<'a>> {
        use crate::Gap;
        use crate::Inset;
        use crate::Max;
        use crate::Min;
        use crate::Plugin::*;
        use PluginType::*;

        let plugin = match self.cmd {
            // stateful plugins require some arg from their subject
            Border(b) => {
                return plugin::border(b, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Rounded(r) => {
                return plugin::rounded(r, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Position(p) => {
                return plugin::position(p, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Visibility(v) => {
                return plugin::visibility(v, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Display(d) => {
                return plugin::display(d, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            TextTransform(tt) => {
                return plugin::text_transform(tt, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            TextDecoration(td) => {
                return plugin::text_decoration(td, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Flex(f) => {
                return plugin::flex(f, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Grid(g) => {
                return plugin::grid(g, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Object(o) => {
                return plugin::object(o, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Whitespace(ws) => {
                return plugin::white_space(ws, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Divide(d) => {
                return plugin::divide(d, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            AlignSelf(align) => {
                return plugin::align_self(align, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Prose(p) => {
                return prose::prose(p, &self.value, theme)
                    .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
            }
            Not(_) => todo!(),

            // all other plugins
            Text => Required(plugin::text),
            Font => Required(plugin::font),
            Shadow => Optional(plugin::shadow),
            Transition => Optional(plugin::transition),
            Placeholder => Required(plugin::placeholder),
            Delay => Required(plugin::delay),
            Duration => Optional(plugin::duration),
            Rotate => Required(plugin::rotate),
            Appearance => Required(plugin::appearance),
            Pointer => Required(plugin::pointer_events),
            Ease => Optional(plugin::ease),
            Order => Required(plugin::order),
            From => Required(plugin::from),
            To => Required(plugin::to),
            Outline => Optional(plugin::outline),
            Mix => Required(plugin::mix),
            Col => Required(plugin::col),
            Row => Required(plugin::row),
            Grow => Optional(plugin::grow),
            Shrink => Optional(plugin::shrink),
            Basis => Required(plugin::basis),
            Italic => Singular(plugin::italic),
            Justify => Required(plugin::justify),
            Items => Required(plugin::items),
            Gap(None) => RequiredArbitrary(plugin::gap),
            Gap(Some(Gap::X)) => Required(plugin::gap_x),
            Gap(Some(Gap::Y)) => Required(plugin::gap_y),
            Cursor => Required(plugin::cursor),
            Scale => Required(plugin::scale),
            Box => Required(plugin::box_),
            Select => Required(plugin::select),
            Overflow => Required(plugin::overflow),
            Top => RequiredArbitrary(plugin::top),
            Bottom => RequiredArbitrary(plugin::bottom),
            Left => RequiredArbitrary(plugin::left),
            Right => RequiredArbitrary(plugin::right),
            Translate => Required(plugin::translate),
            Tracking => RequiredArbitrary(plugin::tracking),
            Invert => Optional(plugin::invert),
            Space => Required(plugin::space),
            Transform => Optional(plugin::transform),
            Opacity => Required(plugin::opacity),
            Blur => Optional(plugin::blur),
            Ring => Optional(plugin::ring),
            Sr => Required(plugin::sr),
            Bg => RequiredArbitrary(plugin::bg),
            H => RequiredArbitrary(plugin::h),
            W => RequiredArbitrary(plugin::w),
            TransformOrigin => Required(plugin::transform_origin),
            P => Required(plugin::p),
            Px => Required(plugin::px),
            Pl => Required(plugin::pl),
            Pr => Required(plugin::pr),
            VerticalAlign => Required(plugin::align),
            Py => Required(plugin::py),
            Pt => Required(plugin::pt),
            Pb => Required(plugin::pb),
            M => Required(plugin::m),
            Mx => Required(plugin::mx),
            Ml => Required(plugin::ml),
            Mr => Required(plugin::mr),
            My => Required(plugin::my),
            Mt => Required(plugin::mt),
            Mb => Required(plugin::mb),
            Z => Required(plugin::z),
            Min(Min::H) => RequiredArbitrary(plugin::min_h),
            Min(Min::W) => RequiredArbitrary(plugin::min_w),
            Max(Max::H) => RequiredArbitrary(plugin::max_h),
            Max(Max::W) => RequiredArbitrary(plugin::max_w),
            Fill => Required(plugin::fill),
            Inset(None) => RequiredArbitrary(plugin::inset),
            Inset(Some(Inset::X)) => RequiredArbitrary(plugin::inset_x),
            Inset(Some(Inset::Y)) => RequiredArbitrary(plugin::inset_y),
            Leading => Required(plugin::leading),
            Truncate => Optional(plugin::truncate),
            Animate => Required(plugin::animation),
        };

        match (plugin, &self.value) {
            (Required(p), Some(SubjectValue::Value(s))) => p(s, theme),
            (Optional(p), Some(SubjectValue::Value(s))) => p(Some(s), theme),
            (Optional(p), None) => p(None, theme),
            (RequiredArbitrary(p), Some(value)) => p(value, theme),
            (OptionalArbitrary(p), value) => p(value.as_ref(), theme),
            (Singular(p), None) => p(),
            _ => None,
        }
        .ok_or_else(|| LiteralConversionError::new(self.cmd, self.value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubjectValue<'a> {
    Value(Value<'a>),
    Css(Css<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Value<'a>(pub &'a str);
#[derive(Debug, PartialEq, Eq)]
pub struct Css<'a>(pub &'a str);

impl Display for SubjectValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectValue::Value(Value(s)) => write!(f, "{s}"),
            SubjectValue::Css(Css(s)) => write!(f, "{s}"),
        }
    }
}

impl<'a> SubjectValue<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            SubjectValue::Value(Value(s)) => s,
            SubjectValue::Css(Css(s)) => s,
        }
    }

    pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, Error<NomSpan<'a>>> {
        match Self::parse_with_span(s) {
            Ok((s, (_, v))) => Ok((s, v)),
            Err(e) => Err(e),
        }
    }

    pub fn parse_with_span(
        s: NomSpan<'a>,
    ) -> IResult<NomSpan<'a>, (NomSpan<'a>, Self), Error<NomSpan<'a>>> {
        // a value is either numeric with dashes signifying fractions,
        // or aplhanumeric with dashes
        let value = verify(Self::parse_value, |s| s.len() > 0)
            .map(|val: NomSpan<'a>| (val, SubjectValue::Value(Value(&val))));
        let css = delimited(char('['), take_while(|c| c != ']'), char(']'))
            .map(|css: NomSpan<'a>| (css, SubjectValue::Css(Css(&css))));

        value.or(css).parse(s)
    }

    /// This algorithm is used to disambiguate between a number with a fraction
    /// and a regular item with an opacity.
    ///
    /// The basic condition is that upon entry of the Number state from the
    /// Neutral State, it is not permitted to leave it. The Number state is
    /// triggered from the Neutral state upon encountering '/' or '.',
    fn parse_value(s: NomSpan<'a>) -> IResult<NomSpan<'a>, NomSpan<'a>, Error<NomSpan<'a>>> {
        #[derive(Debug, PartialEq, Eq)]
        enum Mode {
            /// The initial state, where we can encounter a numbers and dashes.
            Neutral,
            /// We have encountered '/' or '.' and are now expecting a fraction or decimal.
            Number,
            /// We are in 'text mode', and '/' and '.' are not permitted.
            Text,
        }

        let mut state = Mode::Neutral;

        for (i, x) in s.chars().enumerate() {
            match (&state, x) {
                (Mode::Neutral | Mode::Number, '/' | '.') => state = Mode::Number,
                (Mode::Number, x) if is_alphabetic(x as u8) => {
                    return Err(Err::Error(Error::new(s, ErrorKind::AlphaNumeric)))
                }
                (_, x) if is_alphabetic(x as u8) => state = Mode::Text,
                (_, x) if is_digit(x as u8) || x == '-' => {}
                _ => return Ok((s.slice(i..), s.slice(..i))),
            }
        }

        Ok((s.slice(s.len()..), s))
    }
}
