use std::collections::HashMap;

use itertools::Itertools;
use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::{
    Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::{
    config::TailwindTheme,
    infer::{infer_type, Type},
};

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
    match infer_type(theme, rest) {
        Ok(Type::Screen(x)) => Some(to_lit(&[("fontSize", &format!("{}em", x))])),
        Ok(Type::Color(x)) => Some(to_lit(&[("color", x)])),
        _ => None,
    }
}

pub fn font(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_family, rest, "fontFamily", |s| {
        s.iter().join(", ")
    })
}

pub fn shadow(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .box_shadow
        .get(rest)
        .map(|val| {
            to_lit(&[
        ("boxShadow", "var(--tw-shadow)"),
        ("--tw-shadow", &val),
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
    match infer_type(&theme, rest) {
        Ok(Type::Scalar(x)) => Some(to_lit(&[("borderWidth", &format!("{}px", x))])),
        Ok(Type::Color(x)) => Some(to_lit(&[("borderColor", x)])),
        _ => None,
    }
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
    .then_some(to_lit(&[("display", rest)]))
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

pub fn position(rest: &str, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["static", "fixed", "absolute", "relative", "sticky"]
        .contains(&rest)
        .then_some(to_lit(&[("position", rest)]))
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

pub fn bg(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "backgroundColor")
}

pub fn h(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "height")
}

pub fn w(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "width")
}

pub fn p(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "padding")
}

pub fn m(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.spacing, rest, "margin")
}

fn to_lit(items: &[(&str, &str)]) -> ObjectLit {
    ObjectLit {
        span: DUMMY_SP,
        props: items
            .iter()
            .map(|(key, value)| {
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: (*key).into(),
                    }),
                    value: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: (*value).into(),
                    }))),
                })))
            })
            .collect(),
    }
}
