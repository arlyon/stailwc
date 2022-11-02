use swc_core::ecma::ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

use super::nom::{Literal, SubjectValue};

enum PluginType {
    Required(fn(&str, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&str>, &TailwindTheme) -> Option<ObjectLit>),
}

pub fn parse_literal<'a>(theme: &TailwindTheme, lit: Literal<'a>) -> Result<ObjectLit, &'a str> {
    use tailwind_parse::Max;
    use tailwind_parse::Min;
    use tailwind_parse::Plugin::*;
    use PluginType::*;
    let plugin = match lit.cmd {
        Text => Required(plugin::text),
        Font => Required(plugin::font),
        Shadow => Required(plugin::shadow),
        Transition => Optional(plugin::transition),
        Placeholder => Required(plugin::placeholder),
        Delay => Required(plugin::delay),
        Duration => Optional(plugin::duration),
        Divide => Required(plugin::divide),
        Rotate => Required(plugin::rotate),
        Appearance => Required(plugin::appearance),
        Pointer => Required(plugin::pointer_events),
        Ease => Optional(plugin::ease),
        Order => Required(plugin::order),
        Border(_) => Optional(plugin::border),
        Rounded(_) => Optional(plugin::rounded),
        From => Required(plugin::from),
        To => Required(plugin::to),
        Outline => Optional(plugin::outline),
        Mix => Required(plugin::mix),
        Flex => Required(plugin::flex),
        Grid => Required(plugin::grid),
        Col => Required(plugin::col),
        Grow => Optional(plugin::grow),
        Shrink => Optional(plugin::shrink),
        Basis => Required(plugin::basis),
        Justify => Required(plugin::justify),
        Items => Required(plugin::items),
        Gap => Required(plugin::gap),
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
        Position(_) => todo!(),
        Visibility(_) => todo!(),
        Display(_) => todo!(),
        TextTransform(_) => todo!(),
        TextDecoration(_) => todo!(),
    };

    match (plugin, lit.value) {
        (Required(p), Some(SubjectValue::Value(s))) => p(s, theme),
        (Optional(p), Some(SubjectValue::Value(s))) => p(Some(s), theme),
        (Optional(p), None) => p(None, theme),
        _ => None,
    }
    .ok_or(lit.full)
}
