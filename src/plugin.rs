use std::collections::HashMap;

use itertools::Itertools;
use swc_ecma_visit::swc_ecma_ast::ObjectLit;

use crate::{config::TailwindTheme, util::to_lit};

macro_rules! lookup_plugin {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup(&theme.$map, rest, $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup_map(&theme.$map, rest, $target, $closure)
        }
    };
}

macro_rules! lookup_plugin_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup(&theme.$map, rest.unwrap_or("DEFAULT"), $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup_map(&theme.$map, rest.unwrap_or("DEFAULT"), $target, $closure)
        }
    };
}

fn simple_lookup(hashmap: &HashMap<&str, &str>, search: &str, output: &str) -> Option<ObjectLit> {
    hashmap.get(search).map(|val| to_lit(&[(output, val)]))
}

fn simple_lookup_map<V>(
    hashmap: &HashMap<&str, V>,
    search: &str,
    output: &str,
    f: fn(&V) -> String,
) -> Option<ObjectLit> {
    hashmap.get(search).map(|val| to_lit(&[(output, &f(val))]))
}

lookup_plugin_opt!(transition, transition_property, "transitionProperty");
lookup_plugin!(delay, transition_delay, "transitionDelay");
lookup_plugin_opt!(duration, transition_duration, "transitionDuration");
lookup_plugin_opt!(ease, transition_timing_function, "transitionTimingFunction");
lookup_plugin_opt!(blur, blur, "filter", |s| format!("blur({})", s));
lookup_plugin_opt!(invert, invert, "filter", |s| format!("invert({})", s));
lookup_plugin!(basis, flex_basis, "flexBasis");
lookup_plugin_opt!(grow, flex_grow, "flexGrow");
lookup_plugin_opt!(shrink, flex_shrink, "flexShrink");
lookup_plugin!(top, spacing, "top");
lookup_plugin!(opacity, opacity, "opacity");
lookup_plugin!(order, order, "order");
lookup_plugin!(bottom, spacing, "bottom");
lookup_plugin!(left, spacing, "left");
lookup_plugin!(right, spacing, "right");
lookup_plugin!(tracking, letter_spacing, "letterSpacing");
lookup_plugin!(h, height, "height");
lookup_plugin!(w, width, "width");
lookup_plugin!(p, spacing, "padding");
lookup_plugin!(pl, spacing, "paddingLeft");
lookup_plugin!(pr, spacing, "paddingRight");
lookup_plugin!(pt, spacing, "paddingTop");
lookup_plugin!(pb, spacing, "paddingBottom");
lookup_plugin!(m, spacing, "margin");
lookup_plugin!(ml, spacing, "marginLeft");
lookup_plugin!(mr, spacing, "marginRight");
lookup_plugin!(mt, spacing, "marginTop");
lookup_plugin!(mb, spacing, "marginBottom");
lookup_plugin!(z, z_index, "z-index");
lookup_plugin!(gap, gap, "gap");
lookup_plugin_opt!(rounded, border_radius, "borderRadius");
lookup_plugin!(cursor, cursor, "cursor");
lookup_plugin!(scale, scale, "transform", |v| format!("scale({})", v));

pub fn text(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_size, rest, "fontSize", |(a, _)| a.to_string())
        .or_else(|| simple_lookup(&theme.colors, rest, "color"))
        .or_else(|| {
            ["left", "center", "right", "justify", "start", "end"]
                .contains(&rest)
                .then_some(to_lit(&[("textAlign", rest)]))
        })
}

pub fn appearance(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    (rest == "none").then_some(to_lit(&[("appearance", "none")]))
}

pub fn font(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_family, rest, "fontFamily", |s| {
        s.iter().join(", ")
    })
    .or_else(|| simple_lookup(&theme.font_weight, rest, "fontWeight"))
}

pub fn outline(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        None => Some(to_lit(&[("outlineStyle", "solid")])),
        Some("none") => Some(to_lit(&[
            ("outline", "2px solid transparent"),
            ("outlineOffset", "2px"),
        ])),
        Some("dashed") => Some(to_lit(&[("outlineStyle", "dashed")])),
        Some("dotted") => Some(to_lit(&[("outlineStyle", "dotted")])),
        Some("double") => Some(to_lit(&[("outlineStyle", "double")])),
        Some("hidden") => Some(to_lit(&[("outlineStyle", "hidden")])),
        Some(rest) => simple_lookup(&theme.colors, rest, "outlineColor")
            .or_else(|| simple_lookup(&theme.outline_offset, rest, "outlineOffset"))
            .or_else(|| simple_lookup(&theme.outline_width, rest, "outlineWidth")),
    }
}

pub fn bg(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "backgroundColor")
        .or_else(|| simple_lookup(&theme.background_image, rest, "backgroundImage"))
}

pub fn shadow(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .box_shadow
        .get(rest)
        .map(|val| {
            to_lit(&[
        ("boxShadow", "var(--tw-shadow)"),
        ("--tw-shadow", val),
        (
            "--tw-shadow-colored",
            "0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)",
        ),
    ])
        })
        .or_else(|| {
            theme.colors.get(rest).map(|val| {
                to_lit(&[
                    ("--tw-shadow-color", val),
                    ("--tw-shadow", "var(--tw-shadow-colored)"),
                ])
            })
        })
}

pub fn border(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
    rest.and_then(|rest| simple_lookup(&theme.colors, rest, "borderColor"))
        .or_else(|| {
            simple_lookup(
                &theme.border_width,
                rest.unwrap_or("DEFAULT"),
                "borderWidth",
            )
        })
}

pub fn ring(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
    let rest = rest.unwrap_or("DEFAULT");
    match rest.split_once("-") {
        Some(("offset", rest)) => {
            theme.ring_offset_width.get(rest)
                .map(|&s| ("--tw-ring-offset-width", s))
                .or_else(|| theme.colors.get(rest).map(|&s| ("--tw-ring-offset-color", s)))
                .map(|p| to_lit(&[p, ("boxShadow", "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)")]))
        }
        None => (rest == "inset").then(|| to_lit(&[("--tw-ring-inset", "inset")])).or_else(||simple_lookup(&theme.ring_width, rest, "borderWidth")
            .or_else(|| simple_lookup(&theme.colors, rest, "--tw-ring-color"))),
        _ => None,
    }
}

pub fn flex(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "row" => Some("row"),
        "row-reverse" => Some("row-reverse"),
        "col" => Some("column"),
        "col-reverse" => Some("column-reverse"),
        _ => None,
    }
    .map(|v| to_lit(&[("flexDirection", v)]))
    .or_else(|| {
        ["wrap", "wrap-reverse", "nowrap"]
            .contains(&rest)
            .then_some(to_lit(&[("flexWrap", rest)]))
    })
    .or_else(|| simple_lookup(&theme.flex, rest, "flex"))
}

pub fn grid(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    let (cmd, rest) = rest.split_once('-')?;
    match cmd {
        "cols" => simple_lookup(&theme.grid_template_columns, rest, "gridTemplateColumns"),
        "rows" => simple_lookup(&theme.grid_template_rows, rest, "gridTemplateRows"),
        "flow" => {
            let x = match rest {
                "row" => "row",
                "col" => "column",
                "dense" => "dense",
                "row-dense" => "row dense",
                "col-dense" => "column dense",
                _ => return None,
            };

            Some(to_lit(&[("gridAutoFlow", x)]))
        }
        _ => None,
    }
}

pub fn justify(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "start" => Some("flex-start"),
        "end" => Some("flex-end"),
        "center" => Some("center"),
        "between" => Some("between"),
        "around" => Some("around"),
        "evenly" => Some("evenly"),
        _ => None,
    }
    .map(|v| to_lit(&[("justifyContent", v)]))
}

pub fn items(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "start" => Some("flex-start"),
        "end" => Some("flex-end"),
        "center" => Some("center"),
        "baseline" => Some("baseline"),
        "stretch" => Some("stretch"),
        _ => None,
    }
    .map(|v| to_lit(&[("visibility", v)]))
}

pub fn transform(rest: Option<&str>, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[("transform", match rest {
            Some("cpu") | None => "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some("gpu") => "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some("none") => "none",
            _ => return None,
    })]))
}

pub fn display(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    [
        "block",
        "inline-block",
        "inline",
        "flex",
        "inline-flex",
        "table",
        "inline-table",
        "table-caption",
        "table-cell",
        "table-column",
        "table-column-group",
        "table-footer-group",
        "table-header-group",
        "table-row-group",
        "table-row",
        "flow-root",
        "grid",
        "inline-grid",
        "contents",
        "list-item",
        "hidden",
    ]
    .contains(&rest)
    .then_some(to_lit(&[(
        "display",
        if rest == "hidden" { "none" } else { rest },
    )]))
}

pub fn box_(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["border-box", "content-box"]
        .contains(&rest)
        .then_some(to_lit(&[("boxSizing", rest)]))
}

pub fn select(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["none", "text", "all", "auto"]
        .contains(&rest)
        .then_some(to_lit(&[("userSelect", rest)]))
}

pub fn overflow(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["auto", "hidden", "clip", "visible", "scroll"]
        .contains(&rest)
        .then_some(to_lit(&[("overflow", rest)]))
}

pub fn position(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["static", "fixed", "absolute", "relative", "sticky"]
        .contains(&rest)
        .then_some(to_lit(&[("position", rest)]))
}

pub fn visibility(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "visible" => Some("visible"),
        "invisible" => Some("hidden"),
        _ => None,
    }
    .map(|v| to_lit(&[("visibility", v)]))
}

pub fn translate(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    let (cmd, rest) = rest.split_once("-")?;
    match cmd {
        "x" => simple_lookup_map(&theme.translate, rest, "transform", |s| {
            format!("translateX({})", s)
        }),
        "y" => simple_lookup_map(&theme.translate, rest, "transform", |s| {
            format!("translateY({})", s)
        }),
        _ => None,
    }
}
pub fn sr(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    (rest == "only").then(|| {
        to_lit(&[
            ("position", "absolute"),
            ("width", "1px"),
            ("height", "1px"),
            ("padding", "0"),
            ("margin", "-1px"),
            ("overflow", "hidden"),
            ("clip", "rect(0,0,0,0)"),
            ("whiteSpace", "no-wrap"),
            ("borderWidth", "0"),
        ])
    })
}

pub fn px(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("paddingLeft", s), ("paddingRight", s)]))
}

pub fn py(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("paddingTop", s), ("paddingBottom", s)]))
}

pub fn mx(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("marginLeft", s), ("marginRight", s)]))
}

pub fn my(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("marginTop", s), ("marginBottom", s)]))
}
