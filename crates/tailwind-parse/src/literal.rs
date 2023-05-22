use std::boxed::Box as StdBox;
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
use stailwc_swc_utils::to_lit;
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
    InvalidArguments(Plugin, SubjectValue<'a>, Vec<&'a str>),
}

pub type PluginResult<'a> = Result<ObjectLit, Vec<&'a str>>;

/// The types of plugin evaluators that can be used.
enum PluginType<'a, 'func> {
    /// This plugin takes no input, and produces an object literal.
    Singular(fn() -> ObjectLit),
    SingularBox(Box<dyn Fn() -> ObjectLit + 'func>),
    /// This plugin requires a value, and produces an object literal.
    Required(fn(&Value, &'a TailwindTheme) -> PluginResult<'a>),
    #[allow(clippy::type_complexity)]
    RequiredBox(Box<dyn Fn(&Value, &'a TailwindTheme) -> PluginResult<'a> + 'func>),
    #[allow(clippy::type_complexity)]
    OptionalAbitraryBox(
        Box<dyn Fn(Option<&SubjectValue>, &'a TailwindTheme) -> PluginResult<'a> + 'func>,
    ),

    /// This plugin takes an optional value, and produces an object literal.
    Optional(fn(Option<&Value>, &'a TailwindTheme) -> PluginResult<'a>),
    /// This plugin requires a value, or arbitrary css.
    RequiredArbitrary(fn(&SubjectValue, &'a TailwindTheme) -> PluginResult<'a>),
    #[allow(clippy::type_complexity)]
    RequiredArbitraryBox(Box<dyn Fn(&SubjectValue, &'a TailwindTheme) -> PluginResult<'a> + 'func>),
    /// This plugin takes an optional value, or arbitrary css.
    OptionalArbitrary(fn(Option<&SubjectValue>, &'a TailwindTheme) -> PluginResult<'a>),
    RequiredArbitraryTransparency(
        fn(&SubjectValue, &'a TailwindTheme, Option<&Value>) -> PluginResult<'a>,
    ),
    OptionalArbitraryTransparency(
        fn(Option<&SubjectValue>, &'a TailwindTheme, Option<&Value>) -> PluginResult<'a>,
    ),
}

impl<'a> Literal<'a> {
    /// Takes the combination of a plugin and a value and converts it into a
    /// javascript object literal with the equivalent css.
    pub fn to_object_lit(
        self,
        _span: Span,
        theme: &'a TailwindTheme,
        alpha: &Option<Value>,
    ) -> Result<ObjectLit, LiteralConversionError<'a>> {
        use crate::Auto;
        use crate::Gap;
        use crate::Inset;
        use crate::List;
        use crate::Max;
        use crate::Min;
        use crate::Plugin::*;
        use crate::Ring;
        use crate::Text;
        use PluginType::*;

        let plugin = match self.cmd {
            // stateful plugins require some arg from their subject
            Border(b) => OptionalAbitraryBox(StdBox::new(move |v, t| {
                plugin::border(b, v, t, alpha.as_ref())
            })),
            Rounded(r) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::rounded(r, v, t))),
            Position(p) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::position(p, v, t))),
            Visibility(vis) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::visibility(vis, v, t)))
            }
            Display(d) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::display(d, v, t))),
            TextTransform(tt) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::text_transform(tt, v, t)))
            }
            TextDecoration(td) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::text_decoration(td, v, t)))
            }
            Flex(f) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::flex(f, v, t))),
            Grid(g) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::grid(g, v, t))),
            Object(o) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::object(o, v, t))),
            Whitespace(ws) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::white_space(ws, v, t)))
            }
            Divide(d) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::divide(d, v, t))),
            AlignSelf(align) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::align_self(align, v, t)))
            }
            Prose(p) => OptionalAbitraryBox(StdBox::new(move |v, t| prose::prose(p, v, t))),
            Translate(tr) => {
                OptionalAbitraryBox(StdBox::new(move |v, t| plugin::translate(tr, v, t)))
            }
            Col(c) => RequiredArbitraryBox(StdBox::new(move |v, t| plugin::col(c, v, t))),
            Row(r) => RequiredArbitraryBox(StdBox::new(move |v, t| plugin::row(r, v, t))),
            Overflow(o) => RequiredBox(StdBox::new(move |v, t| plugin::overflow(o, v, t))),
            Not(_) => todo!(),

            List(list) => SingularBox(StdBox::new(move || {
                let var = match list {
                    List::None => "none",
                    List::Disc => "disc",
                    List::Decimal => "decimal",
                };
                to_lit(&[("listStyleType", var)])
            })),
            Backdrop(b) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::backdrop(b, v, t))),
            Snap(s) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::snap(s, v, t))),
            Scroll(s) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::scroll(s, v, t))),
            Content(c) => OptionalAbitraryBox(StdBox::new(move |v, t| plugin::content(c, v, t))),

            Auto(Auto::Cols) => RequiredArbitrary(plugin::auto_cols),
            Auto(Auto::Rows) => RequiredArbitrary(plugin::auto_rows),

            // all other plugins
            Text(None) => RequiredArbitraryTransparency(plugin::text),
            Text(Some(Text::Opacity)) => Required(plugin::text_opacity),
            Font => Required(plugin::font),
            Shadow => Optional(plugin::shadow),
            Transition => Optional(plugin::transition),
            Placeholder => RequiredArbitraryTransparency(plugin::placeholder),
            Delay => RequiredArbitrary(plugin::delay),
            Duration => Optional(plugin::duration),
            Decoration => RequiredArbitrary(plugin::decoration),
            Rotate => Required(plugin::rotate),
            Appearance => Required(plugin::appearance),
            Pointer => Required(plugin::pointer_events),
            Ease => Optional(plugin::ease),
            LineClamp => RequiredArbitrary(plugin::line_clamp),
            Order => RequiredArbitrary(plugin::order),
            From => Required(plugin::from),
            To => Required(plugin::to),
            Aspect => RequiredArbitrary(plugin::aspect),
            Outline => OptionalArbitrary(plugin::outline),
            Mix => Required(plugin::mix),
            Grow => OptionalArbitrary(plugin::grow),
            Shrink => OptionalArbitrary(plugin::shrink),
            Basis => RequiredArbitrary(plugin::basis),
            Italic => Singular(plugin::italic),
            Stroke => RequiredArbitrary(plugin::stroke),
            Justify => Required(plugin::justify),
            Items => Required(plugin::items),
            Gap(None) => RequiredArbitrary(plugin::gap),
            Gap(Some(Gap::X)) => RequiredArbitrary(plugin::gap_x),
            Gap(Some(Gap::Y)) => RequiredArbitrary(plugin::gap_y),
            Cursor => Required(plugin::cursor),
            Scale => Required(plugin::scale),
            Box => Required(plugin::box_),
            Select => Required(plugin::select),
            Top => RequiredArbitrary(plugin::top),
            Bottom => RequiredArbitrary(plugin::bottom),
            Antialiased => Singular(plugin::antialiased),
            Left => RequiredArbitrary(plugin::left),
            Right => RequiredArbitrary(plugin::right),
            Tracking => RequiredArbitrary(plugin::tracking),
            Caret => RequiredArbitraryTransparency(plugin::caret),
            Invert => Optional(plugin::invert),
            Float => Required(plugin::float),
            Space => Required(plugin::space),
            Transform => Optional(plugin::transform),
            Opacity => RequiredArbitrary(plugin::opacity),
            Blur => Optional(plugin::blur),
            Ring(None) => OptionalArbitraryTransparency(plugin::ring),
            Ring(Some(Ring::Offset)) => RequiredArbitraryTransparency(plugin::ring_offset),
            Ring(Some(Ring::Opacity)) => RequiredArbitrary(plugin::ring_opacity),
            Ring(Some(Ring::Inset)) => Singular(plugin::ring_inset),
            Sr => Required(plugin::sr),
            Bg => RequiredArbitraryTransparency(plugin::bg),
            H => RequiredArbitrary(plugin::h),
            W => RequiredArbitrary(plugin::w),
            TransformOrigin => Required(plugin::transform_origin),
            P => RequiredArbitrary(plugin::p),
            Px => RequiredArbitrary(plugin::px),
            Pl => RequiredArbitrary(plugin::pl),
            Pr => RequiredArbitrary(plugin::pr),
            VerticalAlign => Required(plugin::align),
            Py => RequiredArbitrary(plugin::py),
            Pt => RequiredArbitrary(plugin::pt),
            Pb => RequiredArbitrary(plugin::pb),
            M => RequiredArbitrary(plugin::m),
            Mx => RequiredArbitrary(plugin::mx),
            Accent => RequiredArbitraryTransparency(plugin::accent),
            Ml => RequiredArbitrary(plugin::ml),
            Mr => RequiredArbitrary(plugin::mr),
            My => RequiredArbitrary(plugin::my),
            Mt => RequiredArbitrary(plugin::mt),
            Mb => RequiredArbitrary(plugin::mb),
            Z => RequiredArbitrary(plugin::z),
            Min(Min::H) => RequiredArbitrary(plugin::min_h),
            Min(Min::W) => RequiredArbitrary(plugin::min_w),
            Max(Max::H) => RequiredArbitrary(plugin::max_h),
            Max(Max::W) => RequiredArbitrary(plugin::max_w),
            Fill => Required(plugin::fill),
            Inset(None) => RequiredArbitrary(plugin::inset),
            Inset(Some(Inset::X)) => RequiredArbitrary(plugin::inset_x),
            Inset(Some(Inset::Y)) => RequiredArbitrary(plugin::inset_y),
            Leading => Required(plugin::leading),
            Truncate => Singular(plugin::truncate),
            Animate => Required(plugin::animation),
            Columns => RequiredArbitrary(plugin::columns),
        };

        match (plugin, &self.value) {
            (Required(p), Some(SubjectValue::Value(s))) => p(s, theme),
            (Optional(p), Some(SubjectValue::Value(s))) => p(Some(s), theme),
            (Optional(p), None) => p(None, theme),
            (RequiredArbitrary(p), Some(value)) => p(value, theme),
            (RequiredArbitraryBox(p), Some(value)) => p(value, theme),
            (Singular(p), None) => Ok(p()),
            (RequiredBox(p), Some(SubjectValue::Value(value))) => p(value, theme),
            (OptionalAbitraryBox(p), value) => p(value.as_ref(), theme),
            (OptionalArbitrary(p), value) => p(value.as_ref(), theme),
            (RequiredArbitraryTransparency(p), Some(value)) => p(value, theme, alpha.as_ref()),
            (OptionalArbitraryTransparency(p), value) => p(value.as_ref(), theme, alpha.as_ref()),
            _ => Err(vec![]),
        }
        .map_err(|e| match self.value {
            Some(v) => LiteralConversionError::InvalidArguments(self.cmd, v, e),
            None => LiteralConversionError::MissingArguments(self.cmd),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubjectValue<'a> {
    Value(Value<'a>),
    Css(Css<'a>),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Value<'a>(pub &'a str);
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    pub fn value(&self) -> Option<&Value> {
        match self {
            SubjectValue::Value(v) => Some(v),
            _ => None,
        }
    }

    pub fn css(&self) -> Option<&Css> {
        match self {
            SubjectValue::Css(s) => Some(s),
            _ => None,
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
        Value::parser()
            .map(|(a, b)| (a, SubjectValue::Value(b)))
            .or(Css::parser().map(|(a, b)| (a, SubjectValue::Css(b))))
            .parse(s)
    }
}

impl<'a> Css<'a> {
    pub fn parser() -> impl Parser<NomSpan<'a>, (NomSpan<'a>, Css<'a>), Error<NomSpan<'a>>> {
        // a value is either numeric with dashes signifying fractions,
        // or aplhanumeric with dashes
        delimited(char('['), take_while(|c| c != ']'), char(']'))
            .map(|css: NomSpan<'a>| (css, Css(&css)))
    }
}

impl<'a> Value<'a> {
    pub fn parser() -> impl Parser<NomSpan<'a>, (NomSpan<'a>, Value<'a>), Error<NomSpan<'a>>> {
        verify(Self::parse_value, |s| s.len() > 0).map(|val: NomSpan<'a>| (val, Value(&val)))
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
