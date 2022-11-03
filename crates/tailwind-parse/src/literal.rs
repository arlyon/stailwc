use swc_core::{common::Span, ecma::ast::ObjectLit};
use tailwind_config::TailwindTheme;

use crate::plugin_impl as plugin;
use crate::Plugin;

/// The core 'rule' of a tailwind directive.
///
/// Example: `text-2xl` is a `Plugin` with a value of `2xl`.
#[derive(Debug, PartialEq, Eq)]
pub struct Literal<'a> {
    pub cmd: Plugin,
    pub value: Option<SubjectValue<'a>>,
    pub span: Option<Span>,
    pub full: &'a str,
}

#[derive(thiserror::Error, Debug)]
pub enum LiteralConversionError<'a> {
    #[error("invalid plugin `{0}`")]
    InvalidPlugin(&'a str),
    #[error("invalid value `{0}` for plugin `{1}`")]
    InvalidValue(&'a str, &'a str),
}

enum PluginType {
    Required(fn(&str, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&str>, &TailwindTheme) -> Option<ObjectLit>),
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
        use LiteralConversionError::InvalidPlugin;
        use PluginType::*;

        let plugin = match self.cmd {
            // stateful plugins require some arg from their subject
            Border(b) => {
                return plugin::border(b, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Rounded(r) => {
                return plugin::rounded(r, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Position(p) => {
                return plugin::position(p, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Visibility(v) => {
                return plugin::visibility(v, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Display(d) => {
                return plugin::display(d, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            TextTransform(tt) => {
                return plugin::text_transform(tt, self.value, theme)
                    .ok_or(InvalidPlugin(self.full))
            }
            TextDecoration(td) => {
                return plugin::text_decoration(td, self.value, theme)
                    .ok_or(InvalidPlugin(self.full))
            }
            Flex(f) => return plugin::flex(f, self.value, theme).ok_or(InvalidPlugin(self.full)),
            Grid(g) => return plugin::grid(g, self.value, theme).ok_or(InvalidPlugin(self.full)),
            Object(o) => {
                return plugin::object(o, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Whitespace(ws) => {
                return plugin::white_space(ws, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            Divide(d) => {
                return plugin::divide(d, self.value, theme).ok_or(InvalidPlugin(self.full))
            }
            AlignSelf(align) => {
                return plugin::align_self(align, self.value, theme).ok_or(InvalidPlugin(self.full))
            }

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
            Grow => Optional(plugin::grow),
            Shrink => Optional(plugin::shrink),
            Basis => Required(plugin::basis),
            Justify => Required(plugin::justify),
            Items => Required(plugin::items),
            Gap(None) => Required(plugin::gap),
            Gap(Some(Gap::X)) => Required(plugin::gap_x),
            Gap(Some(Gap::Y)) => Required(plugin::gap_y),
            Cursor => Required(plugin::cursor),
            Scale => Required(plugin::scale),
            Box => Required(plugin::box_),
            Select => Required(plugin::select),
            Overflow => Required(plugin::overflow),
            Top => Required(plugin::top),
            Bottom => Required(plugin::bottom),
            Left => Required(plugin::left),
            Right => Required(plugin::right),
            Translate => Required(plugin::translate),
            Tracking => Required(plugin::tracking),
            Invert => Optional(plugin::invert),
            Space => Required(plugin::space),
            Transform => Optional(plugin::transform),
            Opacity => Required(plugin::opacity),
            Blur => Optional(plugin::blur),
            Ring => Optional(plugin::ring),
            Sr => Required(plugin::sr),
            Bg => Required(plugin::bg),
            H => Required(plugin::h),
            W => Required(plugin::w),
            P => Required(plugin::p),
            Px => Required(plugin::px),
            Pl => Required(plugin::pl),
            Pr => Required(plugin::pr),
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
            Min(Min::H) => Required(plugin::min_h),
            Min(Min::W) => Required(plugin::min_w),
            Max(Max::H) => Required(plugin::max_h),
            Max(Max::W) => Required(plugin::max_w),
            Fill => Required(plugin::fill),
            Inset(None) => Required(plugin::inset),
            Inset(Some(Inset::X)) => Required(plugin::inset_x),
            Inset(Some(Inset::Y)) => Required(plugin::inset_y),
            Leading => Required(plugin::leading),
            Truncate => Optional(plugin::truncate),
            Animate => Required(plugin::animation),
        };

        match (plugin, self.value) {
            (Required(p), Some(SubjectValue::Value(s) | SubjectValue::Css(s))) => p(s, theme),
            (Optional(p), Some(SubjectValue::Value(s) | SubjectValue::Css(s))) => p(Some(s), theme),
            (Optional(p), None) => p(None, theme),
            _ => None,
        }
        .ok_or(InvalidPlugin(self.full))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubjectValue<'a> {
    Value(&'a str),
    Css(&'a str),
}

impl<'a> SubjectValue<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            SubjectValue::Value(s) => s,
            SubjectValue::Css(s) => s,
        }
    }
}
