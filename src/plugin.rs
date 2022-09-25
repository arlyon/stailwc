use std::collections::HashMap;

use crate::{
    config::TailwindTheme,
    util::{merge_literals, to_lit},
};
use itertools::Itertools;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};

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

macro_rules! merge_plugins {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (None, None) => None,
                (None, Some(a)) => Some(a),
                (Some(b), None) => Some(b),
                (Some(a), Some(b)) => Some(merge_literals(a, b)),
            }
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
lookup_plugin!(to, colors, "--tw-gradient-to");
lookup_plugin!(w, width, "width");
lookup_plugin!(rotate, rotate, "--tw-rotate");
lookup_plugin!(p, padding, "padding");
lookup_plugin!(pl, padding, "paddingLeft");
lookup_plugin!(pr, padding, "paddingRight");
lookup_plugin!(pt, padding, "paddingTop");
lookup_plugin!(pb, padding, "paddingBottom");
lookup_plugin!(m, margin, "margin");
lookup_plugin!(ml, margin, "marginLeft");
lookup_plugin!(mr, margin, "marginRight");
lookup_plugin!(mt, margin, "marginTop");
lookup_plugin!(mb, margin, "marginBottom");
lookup_plugin!(z, z_index, "zIndex");
lookup_plugin!(gap, gap, "gap");
lookup_plugin!(cursor, cursor, "cursor");
lookup_plugin!(scale, scale, "transform", |v| format!("scale({})", v));

pub fn rounded(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.map(|r| {
        r.split_once('-')
            .map(|(a, b)| (a, Some(b)))
            .unwrap_or((r, None))
    }) {
        Some((cmd, rest)) => {
            let cmds = match cmd {
                "t" => ("borderTopLeftRadius", Some("borderTopRightRadius")),
                "b" => ("borderBottomLeftRadius", Some("borderBottomRightRadius")),
                "l" => ("borderTopLeftRadius", Some("borderBottomLeftRadius")),
                "r" => ("borderTopRightRadius", Some("borderBottomRightRadius")),
                "tr" => ("borderTopRightRadius", None),
                "tl" => ("borderTopLeftRadius", None),
                "br" => ("borderBottomRightRadius", None),
                "bl" => ("borderBottomLeftRadius", None),
                x => return simple_lookup(&theme.border_radius, x, "borderRadius"),
            };

            theme
                .border_radius
                .get(rest.unwrap_or("DEFAULT"))
                .map(|lookup| match cmds {
                    (a, Some(b)) => to_lit(&[(a, lookup), (b, lookup)]),
                    (a, None) => to_lit(&[(a, lookup)]),
                })
        }
        // rounded
        None => simple_lookup(&theme.border_radius, "DEFAULT", "borderRadius"),
    }
}

pub fn pointer_events(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "events-none" => Some(to_lit(&[("pointerEvents", "none")])),
        "events-auto" => Some(to_lit(&[("pointerEvents", "auto")])),
        _ => None,
    }
}

pub fn mix(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.split_once('-') {
        Some(("blend", rest)) => blend(rest, theme),
        _ => None,
    }
}

fn blend(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    let modes = [
        "normal",
        "multiply",
        "screen",
        "overlay",
        "darken",
        "lighten",
        "color-dodge",
        "color-burn",
        "hard-light",
        "soft-light",
        "difference",
        "exclusion",
        "hue",
        "saturation",
        "color",
        "luminosity",
        "plus-lighter",
    ];

    modes
        .contains(&rest)
        .then(|| to_lit(&[("mixBlendMode", rest)]))
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

pub fn space(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.split_once('-') {
        Some((xy, "reverse")) => Some(to_lit(&[(
            match xy {
                "x" => "--tw-space-x-reverse",
                "y" => "--tw-space-y-reverse",
                _ => return None,
            },
            "1",
        )])),
        Some(("x", rest)) => theme.space.get(rest).map(|v| {
            to_lit(&[
                ("--tw-space-x-reverse", "0"),
                (
                    "marginRight",
                    &format!("calc({} * var(--tw-space-x-reverse))", v),
                ),
                (
                    "marginLeft",
                    &format!("calc({} * calc(1 - var(--tw-space-x-reverse)))", v),
                ),
            ])
        }),
        Some(("y", rest)) => theme.space.get(rest).map(|v| {
            to_lit(&[
                ("--tw-space-y-reverse", "0"),
                (
                    "marginTop",
                    &format!("calc({} * calc(1 - var(--tw-space-y-reverse)))", v),
                ),
                (
                    "marginBottom",
                    &format!("calc({} * var(--tw-space-y-reverse))", v),
                ),
            ])
        }),
        _ => None,
    }
    .map(|lit| ObjectLit {
        span: DUMMY_SP,
        props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
                span: DUMMY_SP,
                raw: None,
                value: "> :not([hidden]) ~ :not([hidden])".into(),
            }),
            value: Box::new(Expr::Object(lit)),
        })))],
    })
}

pub fn min(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.split_once('-') {
        Some(("h", rest)) => simple_lookup(&theme.min_height, rest, "minHeight"),
        Some(("w", rest)) => simple_lookup(&theme.min_width, rest, "minWidth"),
        _ => None,
    }
}

pub fn max(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.split_once('-') {
        Some(("h", rest)) => simple_lookup(&theme.max_height, rest, "maxHeight"),
        Some(("w", rest)) => simple_lookup(&theme.max_width, rest, "maxWidth"),
        _ => None,
    }
}

pub fn text_transform(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "textTransform",
        match rest {
            "uppercase" => "uppercase",
            "lowercase" => "lowercase",
            "captialize" => "capitalize",
            "normal-case" => "none",
            _ => return None,
        },
    )]))
}

pub fn text_decoration(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "textDecorationLine",
        match rest {
            "underline" => "underline",
            "overline" => "overline",
            "line-through" => "line-through",
            "no-underline" => "none",
            _ => return None,
        },
    )]))
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
        .or_else(|| {
            rest.map(|r| match r.split_once('-') {
                Some((a, b)) => (a, Some(b)),
                _ => (r, None),
            })
            .and_then(|(def, rest)| {
                Some((
                    match def {
                        "x" => border_x as fn(_, _) -> _,
                        "y" => border_y as fn(_, _) -> _,
                        "t" => border_t as fn(_, _) -> _,
                        "b" => border_b as fn(_, _) -> _,
                        "l" => border_l as fn(_, _) -> _,
                        "r" => border_r as fn(_, _) -> _,
                        _ => return None,
                    },
                    rest,
                ))
            })
            .and_then(|(fun, rest)| fun(rest, theme))
        })
}

lookup_plugin_opt!(border_t, border_width, "borderTopWidth");
lookup_plugin_opt!(border_l, border_width, "borderLeftWidth");
lookup_plugin_opt!(border_r, border_width, "borderRightWidth");
lookup_plugin_opt!(border_b, border_width, "borderBottomWidth");
merge_plugins!(border_x, border_l, border_r);
merge_plugins!(border_y, border_t, border_b);

pub fn from(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme.colors.get(rest).map(|c| {
        to_lit(&[
            ("--tw-gradient-from", c),
            ("--tw-gradient-to", c),
            (
                "--tw-gradient-stops",
                "var(--tw-gradient-from), var(--tw-gradient-to)",
            ),
        ])
    })
}

pub fn ring(rest: Option<&str>, theme: &TailwindTheme) -> Option<ObjectLit> {
    let rest = rest.unwrap_or("DEFAULT");
    match rest.split_once('-') {
        Some(("offset", rest)) => {
            theme.ring_offset_width.get(rest)
                .map(|&s| ("--tw-ring-offset-width", s))
                .or_else(|| theme.colors.get(rest).map(|&s| ("--tw-ring-offset-color", s)))
                .map(|p| to_lit(&[p, ("boxShadow", "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)")]))
        }
        Some((_, _)) => simple_lookup(&theme.colors, rest, "--tw-ring-color"),
        None => (rest == "inset").then(|| to_lit(&[("--tw-ring-inset", "inset")])).or_else(||simple_lookup(&theme.ring_width, rest, "borderWidth")),
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

pub fn divide(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.split_once('-').unwrap_or((rest, "DEFAULT")) {
        ("x", "reverse") => Some(to_lit(&[("--tw-divide-x-reverse", "1")])),
        ("y", "reverse") => Some(to_lit(&[("--tw-divide-y-reverse", "1")])),
        ("x", rest) => theme.divide_width.get(rest).map(|v| {
            to_lit(&[
                ("--tw-divide-x-reverse", "0"),
                (
                    "borderRightWidth",
                    &format!("calc({} * var(--tw-divide-x-reverse))", v),
                ),
                (
                    "borderLeftWidth",
                    &format!("calc({} * calc(1 - var(--tw-divide-x-reverse)))", v),
                ),
            ])
        }),

        ("y", rest) => theme.divide_width.get(rest).map(|v| {
            to_lit(&[
                ("--tw-divide-y-reverse", "0"),
                (
                    "borderTopWidth",
                    &format!("calc({} * calc(1 - var(--tw-divide-y-reverse)))", v),
                ),
                (
                    "borderBottomWidth",
                    &format!("calc({} * var(--tw-divide-y-reverse))", v),
                ),
            ])
        }),
        _ => None,
    }
    .map(|lit| ObjectLit {
        span: DUMMY_SP,
        props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
                span: DUMMY_SP,
                raw: None,
                value: "> :not([hidden]) ~ :not([hidden])".into(),
            }),
            value: Box::new(Expr::Object(lit)),
        })))],
    })
}

pub fn placeholder(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "color").map(|lit| ObjectLit {
        span: DUMMY_SP,
        props: vec![Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
                span: DUMMY_SP,
                raw: None,
                value: "::placeholder".into(),
            }),
            value: Box::new(Expr::Object(lit)),
        })
        .into()],
    })
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

pub fn col(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.grid_column, rest, "gridColumn").or_else(|| match rest.split_once('-') {
        Some(("start", rest)) => simple_lookup(&theme.grid_column_start, rest, "gridColumnStart"),
        _ => None,
    })
}

pub fn justify(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        "start" => Some("flex-start"),
        "end" => Some("flex-end"),
        "center" => Some("center"),
        "between" => Some("space-between"),
        "around" => Some("space-around"),
        "evenly" => Some("space-evenly"),
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
    .map(|v| to_lit(&[("alignItems", v)]))
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
    Some(to_lit(&[(
        "boxSizing",
        match rest {
            "border" => "border-box",
            "content" => "content-box",
            _ => return None,
        },
    )]))
}

pub fn select(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["none", "text", "all", "auto"]
        .contains(&rest)
        .then_some(to_lit(&[("userSelect", rest)]))
}

pub fn overflow(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    let values = ["auto", "hidden", "clip", "visible", "scroll"];
    values
        .contains(&rest)
        .then_some(to_lit(&[("overflow", rest)]))
        .or_else(|| match rest.split_once('-') {
            Some(("x", rest)) => values
                .contains(&rest)
                .then_some(to_lit(&[("overflowX", rest)])),
            Some(("y", rest)) => values
                .contains(&rest)
                .then_some(to_lit(&[("overflowY", rest)])),
            _ => None,
        })
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
    let (cmd, rest) = rest.split_once('-')?;
    match cmd {
        "x" => simple_lookup(&theme.translate, rest, "--tw-translate-x"),
        "y" => simple_lookup(&theme.translate, rest, "--tw-translate-y"),
        _ => None,
    }
    .map(|mut l| {
        l.props.push(
            Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident {
                    sym: "transform".into(),
                    span: DUMMY_SP,
                    optional: false,
                }),
                value: Box::new(Expr::Lit(Lit::Str(Str { span: DUMMY_SP, value: "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".into(), raw: None }))),
            })
            .into(),
        );
        l
    })
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
        .padding
        .get(rest)
        .map(|s| to_lit(&[("paddingLeft", s), ("paddingRight", s)]))
}

pub fn py(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .padding
        .get(rest)
        .map(|s| to_lit(&[("paddingTop", s), ("paddingBottom", s)]))
}

pub fn mx(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .margin
        .get(rest)
        .map(|s| to_lit(&[("marginLeft", s), ("marginRight", s)]))
}

pub fn my(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .margin
        .get(rest)
        .map(|s| to_lit(&[("marginTop", s), ("marginBottom", s)]))
}
