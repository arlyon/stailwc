use swc_ecma_visit::swc_ecma_ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

enum PluginType {
    Required(fn(&str, &TailwindTheme) -> Option<ObjectLit>),
    Optional(fn(Option<&str>, &TailwindTheme) -> Option<ObjectLit>),
}

pub fn parse_literal<'a>(theme: &TailwindTheme, s: &'a str) -> Result<ObjectLit, &'a str> {
    let (cmd, rest) = match s.split_once('-') {
        Some((cmd, rest)) => (cmd, Some(rest)),
        None => {
            let root_plugins = [plugin::position, plugin::visibility, plugin::display];
            match root_plugins.iter().find_map(|p| p(s, theme)) {
                Some(r) => return Ok(r),
                None => (s, None),
            }
        }
    };

    use PluginType::*;
    let plugin = match cmd {
        "text" => Required(plugin::text),
        "font" => Required(plugin::font),
        "shadow" => Required(plugin::shadow),
        "transition" => Optional(plugin::transition),
        "delay" => Required(plugin::delay),
        "duration" => Optional(plugin::duration),
        "ease" => Optional(plugin::ease),
        "rounded" => Optional(plugin::rounded),
        "flex" => Required(plugin::flex),
        "grid" => Required(plugin::grid),
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
        "sr" => Required(plugin::sr),
        "bg" => Required(plugin::bg),
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
        _ => return Err(s),
    };

    match (plugin, rest) {
        (Required(p), Some(s)) => p(s, theme),
        (Optional(p), s) => p(s, theme),
        _ => None,
    }
    .ok_or(s)
}
