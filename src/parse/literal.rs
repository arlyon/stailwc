use swc_ecma_visit::swc_ecma_ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

pub fn parse_literal<'a>(theme: &TailwindTheme, s: &'a str) -> Result<ObjectLit, &'a str> {
    let (command, rest) = s.split_once("-").ok_or(s)?;

    let plugin = match command {
        "text" => plugin::text,
        "font" => plugin::font,
        "shadow" => plugin::shadow,
        "transition" => plugin::transition,
        "delay" => plugin::delay,
        "duration" => plugin::duration,
        "ease" => plugin::ease,
        "border" => plugin::border,
        "rounded" => plugin::rounded,
        "cursor" => plugin::cursor,
        "scale" => plugin::scale,
        "display" => plugin::display,
        "box" => plugin::box_,
        "select" => plugin::select,
        "bg" => plugin::bg,
        "h" => plugin::h,
        "w" => plugin::w,
        "p" => plugin::p,
        "m" => plugin::m,
        _ => return Err(s),
    };

    plugin(rest, theme).ok_or(s)
}
