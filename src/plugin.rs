use std::collections::HashMap;

use itertools::Itertools;
use swc_ecma_visit::swc_ecma_ast::ObjectLit;

use crate::{config::TailwindTheme, util::to_lit};

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

pub fn text(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_size, rest, "fontSize", |(a, _)| a.to_string())
        .or_else(|| simple_lookup(&theme.colors, rest, "color"))
        .or_else(|| {
            ["left", "center", "right", "justify", "start", "end"]
                .contains(&rest)
                .then_some(to_lit(&[("textAlign", rest)]))
        })
}

pub fn font(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_family, rest, "fontFamily", |s| {
        s.iter().join(", ")
    })
    .or_else(|| simple_lookup(&theme.font_weight, rest, "fontWeight"))
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

pub fn transition(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.transition_property, rest, "transitionProperty")
}

pub fn delay(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.transition_delay, rest, "transitionDelay")
}

pub fn duration(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.transition_duration, rest, "transitionDuration")
}

pub fn ease(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(
        &theme.transition_timing_function,
        rest,
        "transitionTimingFunction",
    )
}

pub fn border(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "borderColor")
        .or_else(|| simple_lookup(&theme.border_width, rest, "borderWidth"))
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

pub fn basis(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.flex_basis, rest, "flexBasis")
}

pub fn grow(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.flex_grow, rest, "flexGrow")
}

pub fn shrink(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.flex_shrink, rest, "flexShrink")
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

pub fn gap(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.gap, rest, "gap")
}

pub fn rounded(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.border_radius, rest, "borderRadius")
}

pub fn cursor(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.cursor, rest, "cursor")
}

pub fn scale(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.scale, rest, "transform", |v| format!("scale({})", v))
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

pub fn overflow(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
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

pub fn top(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "top")
}

pub fn bottom(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "bottom")
}

pub fn left(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "left")
}

pub fn right(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "right")
}

pub fn translate(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    println!("processing translate {}", rest);
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

pub fn tracking(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.letter_spacing, rest, "letterSpacing")
}

pub fn bg(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "backgroundColor")
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

pub fn h(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.height, rest, "height")
}

pub fn w(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.width, rest, "width")
}

pub fn p(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "padding")
}

pub fn px(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("paddingLeft", s), ("paddingRight", s)]))
}

pub fn pl(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "paddingLeft")
}

pub fn pr(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "paddingRight")
}

pub fn py(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("paddingTop", s), ("paddingBottom", s)]))
}

pub fn pt(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "paddingTop")
}

pub fn pb(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "paddingBottom")
}

pub fn m(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "margin")
}

pub fn mx(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("marginLeft", s), ("marginRight", s)]))
}

pub fn ml(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "marginLeft")
}

pub fn mr(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "marginRight")
}

pub fn my(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .spacing
        .get(rest)
        .map(|s| to_lit(&[("marginTop", s), ("marginBottom", s)]))
}

pub fn mt(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "marginTop")
}

pub fn mb(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "marginBottom")
}

pub fn z(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.z_index, rest, "z-index")
}
