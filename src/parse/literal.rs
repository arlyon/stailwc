use swc_ecma_visit::swc_ecma_ast::ObjectLit;

use crate::{config::TailwindTheme, plugin};

pub fn parse_literal<'a>(theme: &TailwindTheme, s: &'a str) -> Result<ObjectLit, &'a str> {
    let (command, rest) = match s.split_once('-') {
        Some(s) => s,
        None => {
            let root_plugins = [plugin::position, plugin::visibility, plugin::display];
            return root_plugins.iter().find_map(|p| p(s, theme)).ok_or(s);
        }
    };

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
        "flex" => plugin::flex,
        "grid" => plugin::grid,
        "grow" => plugin::grow,
        "shrink" => plugin::shrink,
        "basis" => plugin::basis,
        "justify" => plugin::justify,
        "items" => plugin::items,
        "gap" => plugin::gap,
        "cursor" => plugin::cursor,
        "scale" => plugin::scale,
        "box" => plugin::box_,
        "select" => plugin::select,
        "overflow" => plugin::overflow,
        "top" => plugin::top,
        "bottom" => plugin::bottom,
        "left" => plugin::left,
        "right" => plugin::right,
        "translate" => plugin::translate,
        "sr" => plugin::sr,
        "bg" => plugin::bg,
        "h" => plugin::h,
        "w" => plugin::w,
        "p" => plugin::p,
        "px" => plugin::px,
        "pl" => plugin::pl,
        "pr" => plugin::pr,
        "py" => plugin::py,
        "pt" => plugin::pt,
        "pb" => plugin::pb,
        "m" => plugin::m,
        "mx" => plugin::mx,
        "ml" => plugin::ml,
        "mr" => plugin::mr,
        "my" => plugin::my,
        "mt" => plugin::mt,
        "mb" => plugin::mb,
        "z" => plugin::z,
        _ => return Err(s),
    };

    plugin(rest, theme).ok_or(s)
}
