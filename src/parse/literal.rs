use swc_core::ecma::ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

use super::nom::{Literal, SubjectValue};

enum PluginType {
    Required(fn(&str, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&str>, &TailwindTheme) -> Option<ObjectLit>),
}

pub fn parse_literal<'a>(theme: &TailwindTheme, lit: Literal<'a>) -> Result<ObjectLit, &'a str> {
    let root_plugins = [
        plugin::position,
        plugin::visibility,
        plugin::display,
        plugin::text_transform,
        plugin::text_decoration,
    ];

    if let None = lit.value {
        if let Some(r) = root_plugins.iter().find_map(|p| p(lit.cmd, theme)) {
            return Ok(r);
        }
    }

    use PluginType::*;
    let plugin = match lit.cmd {
        "text" => Required(plugin::text),
        "font" => Required(plugin::font),
        "shadow" => Required(plugin::shadow),
        "transition" => Optional(plugin::transition),
        "placeholder" => Required(plugin::placeholder),
        "delay" => Required(plugin::delay),
        "duration" => Optional(plugin::duration),
        "divide" => Required(plugin::divide),
        "rotate" => Required(plugin::rotate),
        "appearance" => Required(plugin::appearance),
        "pointer" => Required(plugin::pointer_events),
        "ease" => Optional(plugin::ease),
        "order" => Required(plugin::order),
        "border" => Optional(plugin::border),
        "rounded" => Optional(plugin::rounded),
        "from" => Required(plugin::from),
        "to" => Required(plugin::to),
        "outline" => Optional(plugin::outline),
        "mix" => Required(plugin::mix),
        "flex" => Required(plugin::flex),
        "grid" => Required(plugin::grid),
        "col" => Required(plugin::col),
        "grow" => Optional(plugin::grow),
        "shrink" => Optional(plugin::shrink),
        "basis" => Required(plugin::basis),
        "justify" => Required(plugin::justify),
        "items" => Required(plugin::items),
        "gap" => Required(plugin::gap),
        "cursor" => Required(plugin::cursor),
        "scale" => Required(plugin::scale),
        "box" => Required(plugin::box_),
        "select" => Required(plugin::select),
        "overflow" => Required(plugin::overflow),
        "top" => Required(plugin::top),
        "bottom" => Required(plugin::bottom),
        "left" => Required(plugin::left),
        "right" => Required(plugin::right),
        "translate" => Required(plugin::translate),
        "tracking" => Required(plugin::tracking),
        "invert" => Optional(plugin::invert),
        "space" => Required(plugin::space),
        "transform" => Optional(plugin::transform),
        "opacity" => Required(plugin::opacity),
        "blur" => Optional(plugin::blur),
        "ring" => Optional(plugin::ring),
        "sr" => Required(plugin::sr),
        "bg" => Required(plugin::bg),
        "min" => Required(plugin::min),
        "max" => Required(plugin::max),
        "h" => Required(plugin::h),
        "w" => Required(plugin::w),
        "p" => Required(plugin::p),
        "px" => Required(plugin::px),
        "pl" => Required(plugin::pl),
        "pr" => Required(plugin::pr),
        "py" => Required(plugin::py),
        "pt" => Required(plugin::pt),
        "pb" => Required(plugin::pb),
        "m" => Required(plugin::m),
        "mx" => Required(plugin::mx),
        "ml" => Required(plugin::ml),
        "mr" => Required(plugin::mr),
        "my" => Required(plugin::my),
        "mt" => Required(plugin::mt),
        "mb" => Required(plugin::mb),
        "z" => Required(plugin::z),
        _ => return Err(lit.full),
    };

    match (plugin, lit.value) {
        (Required(p), Some(SubjectValue::Value(s))) => p(s, theme),
        (Optional(p), Some(SubjectValue::Value(s))) => p(Some(s), theme),
        (Optional(p), None) => p(None, theme),
        _ => None,
    }
    .ok_or(lit.full)
}
