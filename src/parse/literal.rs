use swc_core::ecma::ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

use super::nom::{Literal, SubjectValue};

enum PluginType {
    Required(fn(&str, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&str>, &TailwindTheme) -> Option<ObjectLit>),
}

pub fn parse_literal<'a>(theme: &TailwindTheme, lit: Literal<'a>) -> Result<ObjectLit, &'a str> {
    use tailwind_parse::Gap;
    use tailwind_parse::Inset;
    use tailwind_parse::Max;
    use tailwind_parse::Min;
    use tailwind_parse::Plugin::*;
    use PluginType::*;

    let plugin = match lit.cmd {
        // stateful plugins require some arg from their subject
        Border(b) => return plugin::border(b, lit.value, theme).ok_or(lit.full),
        Rounded(r) => return plugin::rounded(r, lit.value, theme).ok_or(lit.full),
        Position(p) => return plugin::position(p, lit.value, theme).ok_or(lit.full),
        Visibility(v) => return plugin::visibility(v, lit.value, theme).ok_or(lit.full),
        Display(d) => return plugin::display(d, lit.value, theme).ok_or(lit.full),
        TextTransform(tt) => return plugin::text_transform(tt, lit.value, theme).ok_or(lit.full),
        TextDecoration(td) => return plugin::text_decoration(td, lit.value, theme).ok_or(lit.full),
        Flex(f) => return plugin::flex(f, lit.value, theme).ok_or(lit.full),
        Grid(g) => return plugin::grid(g, lit.value, theme).ok_or(lit.full),
        Object(o) => return plugin::object(o, lit.value, theme).ok_or(lit.full),
        Whitespace(ws) => return plugin::white_space(ws, lit.value, theme).ok_or(lit.full),
        Divide(d) => return plugin::divide(d, lit.value, theme).ok_or(lit.full),
        AlignSelf(align) => return plugin::align_self(align, lit.value, theme).ok_or(lit.full),

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
    };

    match (plugin, lit.value) {
        (Required(p), Some(SubjectValue::Value(s) | SubjectValue::Css(s))) => p(s, theme),
        (Optional(p), Some(SubjectValue::Value(s) | SubjectValue::Css(s))) => p(Some(s), theme),
        (Optional(p), None) => p(None, theme),
        _ => None,
    }
    .ok_or(lit.full)
}
