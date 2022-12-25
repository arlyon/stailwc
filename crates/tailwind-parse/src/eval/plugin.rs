use std::collections::HashMap;

use crate::{
    AlignSelf, Border, Col, Css, Display, Divide, Flex, Grid, Object, Overflow, PluginResult,
    Position, Rounded, Row, SubjectValue, TextDecoration, TextTransform, Translate, Value,
    Visibility, Whitespace,
};
use itertools::Itertools;
use stailwc_swc_utils::{merge_literals, to_lit};
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};
use tailwind_config::TailwindTheme;

macro_rules! lookup_plugin {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup(&theme.$map, rest, $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup_map(&theme.$map, rest, $target, $closure)
        }
    };
}

macro_rules! array_plugin {
    ($def:ident, $options:expr, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
            $options
                .iter()
                .find(|&x| x == rest)
                .map(|_| to_lit(&[($target, rest)]))
                .ok_or_else(|| vec![])
        }
    };
}

macro_rules! array_map_plugin {
    ($def:ident, $options:expr, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
            $options
                .iter()
                .find(|(x, _)| x == rest)
                .map(|(_, y)| to_lit(&[($target, y)]))
                .ok_or_else(|| vec![])
        }
    };
}

macro_rules! lookup_plugin_arbitrary {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(value: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => simple_lookup(&theme.$map, v, $target),
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(value: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
}

macro_rules! lookup_plugin_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup(&theme.$map, rest.map(|v| v.0).unwrap_or("DEFAULT"), $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup_map(
                &theme.$map,
                rest.map(|v| v.0).unwrap_or("DEFAULT"),
                $target,
                $closure,
            )
        }
    };
}

macro_rules! lookup_plugin_arbitrary_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(
            value: Option<&SubjectValue>,
            theme: &'a TailwindTheme,
        ) -> PluginResult<'a> {
            match value {
                Some(SubjectValue::Value(Value(v))) => simple_lookup(&theme.$map, v, $target),
                Some(SubjectValue::Css(Css(v))) => Ok(to_lit(&[($target, v)])),
                // if there is no value, attempt to look up the default
                None => simple_lookup(&theme.$map, "DEFAULT", $target),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(
            value: Option<&SubjectValue>,
            theme: &'a TailwindTheme,
        ) -> PluginResult<'a> {
            match value {
                Some(SubjectValue::Value(Value(v))) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                Some(SubjectValue::Css(Css(v))) => to_lit(&[($target, v)]),
                // if there is no value, attempt to look up the default
                None => simple_lookup_map(&theme.$map, "DEFAULT", $target, $closure),
            }
        }
    };
}

macro_rules! merge_plugins {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

macro_rules! merge_plugins_arbitrary {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! merge_plugins_opt {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (None, None) => None,
                (None, Some(a)) => Some(a),
                (Some(b), None) => Some(b),
                (Some(a), Some(b)) => Some(merge_literals(a, b)),
            }
        }
    };
}

macro_rules! merge_plugins_arbitrary_opt {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: Option<&SubjectValue>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

fn simple_lookup<'a>(
    hashmap: &HashMap<&'a str, &str>,
    search: &str,
    output: &str,
) -> PluginResult<'a> {
    hashmap
        .get(search)
        .map(|val| to_lit(&[(output, val)]))
        .ok_or_else(|| {
            let sort = eddie::Levenshtein::new();
            hashmap
                .keys()
                .sorted_by_key(|val| sort.distance(search, val))
                .copied()
                .take(5)
                .collect()
        })
}

fn simple_lookup_map<'a, V>(
    hashmap: &HashMap<&'a str, V>,
    search: &str,
    output: &str,
    f: fn(&V) -> String,
) -> PluginResult<'a> {
    hashmap
        .get(search)
        .map(|val| to_lit(&[(output, &f(val))]))
        .ok_or_else(Vec::new)
}

lookup_plugin_opt!(transition, transition_property, "transitionProperty");
lookup_plugin!(delay, transition_delay, "transitionDelay");
lookup_plugin_opt!(duration, transition_duration, "transitionDuration");
lookup_plugin_opt!(ease, transition_timing_function, "transitionTimingFunction");
lookup_plugin_opt!(blur, blur, "filter", |s| format!("blur({s})"));
lookup_plugin_opt!(invert, invert, "filter", |s| format!("invert({s})"));
lookup_plugin!(basis, flex_basis, "flexBasis");
lookup_plugin_opt!(grow, flex_grow, "flexGrow");
lookup_plugin_opt!(shrink, flex_shrink, "flexShrink");
lookup_plugin_arbitrary!(top, height, "top");
lookup_plugin!(opacity, opacity, "opacity");
lookup_plugin!(animation, animation, "animation");
lookup_plugin!(order, order, "order");
lookup_plugin_arbitrary!(bottom, height, "bottom");
lookup_plugin!(fill, colors, "fill");
lookup_plugin_arbitrary!(left, width, "left");
lookup_plugin_arbitrary!(right, width, "right");
lookup_plugin_arbitrary!(tracking, letter_spacing, "letterSpacing");
lookup_plugin_arbitrary!(h, height, "height");
lookup_plugin!(ring_width, ring_width, "borderWidth");
lookup_plugin!(ring_color, colors, "--tw-ring-color");
merge_plugins!(ring_base, ring_width, ring_color);
lookup_plugin!(to, colors, "--tw-gradient-to");
lookup_plugin_arbitrary!(w, width, "width");
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
lookup_plugin_arbitrary!(gap, gap, "gap");
lookup_plugin!(gap_x, gap, "columnGap");
lookup_plugin!(gap_y, gap, "rowGap");
lookup_plugin!(cursor, cursor, "cursor");
lookup_plugin!(scale, scale, "transform", |v| format!("scale({v})"));

lookup_plugin_arbitrary!(min_w, min_width, "minWidth");
lookup_plugin_arbitrary!(max_w, max_width, "maxWidth");
lookup_plugin_arbitrary!(min_h, min_height, "minHeight");
lookup_plugin_arbitrary!(max_h, max_height, "maxHeight");

lookup_plugin!(leading, line_height, "lineHeight");

array_plugin!(
    align,
    [
        "baseline",
        "top",
        "middle",
        "bottom",
        "text-top",
        "text-bottom",
        "sub",
        "super",
    ],
    "verticalAlign"
);
array_map_plugin!(
    pointer_events,
    [("events-none", "none"), ("events-auto", "auto")],
    "pointerEvents"
);
array_plugin!(float, ["left", "right", "none"], "float");

array_plugin!(
    blend,
    [
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
        "plus-lighter"
    ],
    "mixBlendMode"
);

array_plugin!(
    text_align,
    ["left", "center", "right", "justify", "start", "end"],
    "textAlign"
);
lookup_plugin_arbitrary!(text_color, colors, "color");
lookup_plugin_arbitrary!(text_size, font_size, "fontSize", |t| t.0.to_string());

array_plugin!(appearance, ["none"], "appearance");

lookup_plugin!(font_family, font_family, "fontFamily", |s| s
    .iter()
    .join(" "));
lookup_plugin!(font_weight, font_weight, "fontWeight");
merge_plugins!(font, font_family, font_weight);

array_map_plugin!(
    bg_repeat,
    [
        ("repeat", "repeat"),
        ("repeat-x", "repeat-x"),
        ("repeat-y", "repeat-y"),
        ("no-repeat", "no-repeat"),
        ("repeat-round", "round"),
        ("repeat-space", "space"),
    ],
    "backgroundRepeat"
);
lookup_plugin_arbitrary_opt!(border_color, colors, "borderColor");
lookup_plugin_arbitrary_opt!(border_width, border_width, "borderWidth");
lookup_plugin_arbitrary_opt!(border_t, border_width, "borderTopWidth");
lookup_plugin_arbitrary_opt!(border_l, border_width, "borderLeftWidth");
lookup_plugin_arbitrary_opt!(border_r, border_width, "borderRightWidth");
lookup_plugin_arbitrary_opt!(border_b, border_width, "borderBottomWidth");
merge_plugins_arbitrary_opt!(border_x, border_l, border_r);
merge_plugins_arbitrary_opt!(border_y, border_t, border_b);
merge_plugins_arbitrary_opt!(border_cw, border_color, border_width);
merge_plugins_arbitrary_opt!(border_inner, border_cw, border_style);

merge_plugins_arbitrary!(inset_x, left, right);
merge_plugins_arbitrary!(inset_y, top, bottom);
merge_plugins_arbitrary!(inset, inset_x, inset_y);

lookup_plugin_arbitrary!(grid_t_col, grid_template_columns, "gridTemplateColumns");
lookup_plugin_arbitrary!(grid_t_row, grid_template_rows, "gridTemplateRows");

lookup_plugin!(grid_col, grid_column, "gridColumn");
lookup_plugin!(grid_col_start, grid_column_start, "gridColumnStart");
lookup_plugin!(grid_col_end, grid_column_end, "gridColumnEnd");

lookup_plugin!(grid_row, grid_row, "gridRow");
lookup_plugin!(grid_row_start, grid_row_start, "gridRowStart");
lookup_plugin!(grid_row_end, grid_row_end, "gridRowEnd");

array_map_plugin!(
    items,
    [
        ("start", "flex-start"),
        ("end", "flex-end"),
        ("center", "center"),
        ("baseline", "baseline"),
        ("stretch", "stretch")
    ],
    "alignItems"
);
array_map_plugin!(
    box_,
    [("border", "border-box"), ("content", "content-box")],
    "boxSizing"
);

array_plugin!(select, ["none", "text", "all", "auto"], "userSelect");

array_plugin!(
    overflow_base,
    ["auto", "hidden", "clip", "visible", "scroll"],
    "overflow"
);

array_plugin!(
    overflow_x,
    ["auto", "hidden", "clip", "visible", "scroll"],
    "overflowX"
);

array_plugin!(
    overflow_y,
    ["auto", "hidden", "clip", "visible", "scroll"],
    "overflowY"
);

lookup_plugin_arbitrary!(translate_x, translate, "--tw-translate-x");
lookup_plugin_arbitrary!(translate_y, translate, "--tw-translate-y");

merge_plugins!(my, mt, mb);
merge_plugins!(mx, ml, mr);
merge_plugins!(py, pt, pb);
merge_plugins!(px, pl, pr);

pub fn rounded<'a>(
    subcommand: Option<Rounded>,
    rest: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let cmds = match subcommand {
        None => {
            return match rest {
                Some(SubjectValue::Value(Value(v))) => {
                    simple_lookup(&theme.border_radius, v, "borderRadius")
                }
                Some(SubjectValue::Css(Css(v))) => Ok(to_lit(&[("borderRadius", v)])),
                None => simple_lookup(&theme.border_radius, "DEFAULT", "borderRadius"),
            }
        }
        Some(Rounded::T) => ("borderTopLeftRadius", Some("borderTopRightRadius")),
        Some(Rounded::B) => ("borderBottomLeftRadius", Some("borderBottomRightRadius")),
        Some(Rounded::L) => ("borderTopLeftRadius", Some("borderBottomLeftRadius")),
        Some(Rounded::R) => ("borderTopRightRadius", Some("borderBottomRightRadius")),
        Some(Rounded::Tr) => ("borderTopRightRadius", None),
        Some(Rounded::Tl) => ("borderTopLeftRadius", None),
        Some(Rounded::Br) => ("borderBottomRightRadius", None),
        Some(Rounded::Bl) => ("borderBottomLeftRadius", None),
    };

    let radius = match rest {
        Some(SubjectValue::Value(Value(v))) => theme.border_radius.get(v),
        Some(SubjectValue::Css(Css(v))) => Some(v),
        None => theme.border_radius.get("DEFAULT"),
    };

    radius
        .map(|lookup| match cmds {
            (a, Some(b)) => to_lit(&[(a, lookup), (b, lookup)]),
            (a, None) => to_lit(&[(a, lookup)]),
        })
        .ok_or(vec![])
}

pub fn mix<'a>(rest: &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match rest.0.split_once('-') {
        Some(("blend", rest)) => blend(&Value(rest), theme),
        _ => Err(vec![]),
    }
}

pub fn transform_origin<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    match *rest {
        "center" => Ok(to_lit(&[("transformOrigin", "center")])),
        "top" => Ok(to_lit(&[("transformOrigin", "top")])),
        "top-right" => Ok(to_lit(&[("transformOrigin", "top right")])),
        "right" => Ok(to_lit(&[("transformOrigin", "right")])),
        "bottom-right" => Ok(to_lit(&[("transformOrigin", "bottom right")])),
        "bottom" => Ok(to_lit(&[("transformOrigin", "bottom")])),
        "bottom-left" => Ok(to_lit(&[("transformOrigin", "bottom left")])),
        "left" => Ok(to_lit(&[("transformOrigin", "left")])),
        "top-left" => Ok(to_lit(&[("transformOrigin", "top left")])),
        _ => Err(vec![]),
    }
}

pub fn text<'a>(value: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
    text_size(value, theme)
        .or_else(|_e| text_color(value, theme))
        .or_else(|e| match value {
            SubjectValue::Value(v) => text_align(v, theme),
            SubjectValue::Css(_) => Err(e),
        })
}

pub fn space<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match rest.split_once('-') {
        Some((xy, "reverse")) => Ok(to_lit(&[(
            match xy {
                "x" => "--tw-space-x-reverse",
                "y" => "--tw-space-y-reverse",
                _ => return Err(vec![]),
            },
            "1",
        )])),
        Some(("x", rest)) => theme
            .space
            .get(rest)
            .map(|v| {
                to_lit(&[
                    ("--tw-space-x-reverse", "0"),
                    (
                        "marginRight",
                        &format!("calc({v} * var(--tw-space-x-reverse))"),
                    ),
                    (
                        "marginLeft",
                        &format!("calc({v} * calc(1 - var(--tw-space-x-reverse)))"),
                    ),
                ])
            })
            .ok_or(vec![]),
        Some(("y", rest)) => theme
            .space
            .get(rest)
            .map(|v| {
                to_lit(&[
                    ("--tw-space-y-reverse", "0"),
                    (
                        "marginTop",
                        &format!("calc({v} * calc(1 - var(--tw-space-y-reverse)))"),
                    ),
                    (
                        "marginBottom",
                        &format!("calc({v} * var(--tw-space-y-reverse))"),
                    ),
                ])
            })
            .ok_or(vec![]),
        _ => Err(vec![]),
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

pub fn text_transform<'a>(
    tt: TextTransform,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    Ok(to_lit(&[(
        "textTransform",
        match tt {
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
            TextTransform::NormalCase => "none",
        },
    )]))
}

pub fn text_decoration<'a>(
    td: TextDecoration,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    Ok(to_lit(&[(
        "textDecorationLine",
        match td {
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::NoUnderline => "none",
        },
    )]))
}

pub fn truncate() -> ObjectLit {
    to_lit(&[
        ("overflow", "hidden"),
        ("textOverflow", "ellipsis"),
        ("whiteSpace", "nowrap"),
    ])
}

pub fn outline<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match rest {
        None => Ok(to_lit(&[("outlineStyle", "solid")])),
        Some(Value("none")) => Ok(to_lit(&[
            ("outline", "2px solid transparent"),
            ("outlineOffset", "2px"),
        ])),
        Some(Value("dashed")) => Ok(to_lit(&[("outlineStyle", "dashed")])),
        Some(Value("dotted")) => Ok(to_lit(&[("outlineStyle", "dotted")])),
        Some(Value("double")) => Ok(to_lit(&[("outlineStyle", "double")])),
        Some(Value("hidden")) => Ok(to_lit(&[("outlineStyle", "hidden")])),
        Some(rest) => simple_lookup(&theme.colors, rest.0, "outlineColor")
            .or_else(|_e| simple_lookup(&theme.outline_offset, rest.0, "outlineOffset"))
            .or_else(|_e| simple_lookup(&theme.outline_width, rest.0, "outlineWidth")),
    }
}

pub fn bg<'a>(val: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match val {
        SubjectValue::Value(Value(rest)) => simple_lookup(&theme.colors, rest, "backgroundColor")
            .or_else(|_e| simple_lookup(&theme.background_image, rest, "backgroundImage"))
            .or_else(|_e| simple_lookup(&theme.background_size, rest, "backgroundSize"))
            .or_else(|_e| simple_lookup(&theme.background_position, rest, "backgroundPosition"))
            .or_else(|_e| bg_repeat(&Value(rest), theme)),
        SubjectValue::Css(Css(css)) => Ok(to_lit(&[("background", css)])),
    }
}

pub fn shadow<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
    let lookup = rest.map(|v| v.0).unwrap_or("DEFAULT");
    theme
        .box_shadow
        .get(lookup)
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
            theme.colors.get(lookup).map(|val| {
                to_lit(&[
                    ("--tw-shadow-color", val),
                    ("--tw-shadow", "var(--tw-shadow-colored)"),
                ])
            })
        })
        .ok_or(vec![])
}

pub fn border<'a>(
    subcommand: Option<Border>,
    rest: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let func = match subcommand {
        None => border_inner,
        Some(Border::T) => border_t,
        Some(Border::B) => border_b,
        Some(Border::L) => border_l,
        Some(Border::R) => border_r,
        Some(Border::X) => border_x,
        Some(Border::Y) => border_y,
    };

    func(rest.as_ref(), theme)
}

fn border_style(rest: Option<&SubjectValue>, _theme: &TailwindTheme) -> PluginResult<'static> {
    match rest {
        Some(SubjectValue::Value(Value("solid"))) => Ok(to_lit(&[("borderStyle", "solid")])),
        Some(SubjectValue::Value(Value("dashed"))) => Ok(to_lit(&[("borderStyle", "dashed")])),
        Some(SubjectValue::Value(Value("dotted"))) => Ok(to_lit(&[("borderStyle", "dotted")])),
        Some(SubjectValue::Value(Value("double"))) => Ok(to_lit(&[("borderStyle", "double")])),
        Some(SubjectValue::Value(Value("hidden"))) => Ok(to_lit(&[("borderStyle", "hidden")])),
        Some(SubjectValue::Value(Value("none"))) => Ok(to_lit(&[("borderStyle", "none")])),
        _ => Err(vec![]),
    }
}

pub fn from(Value(rest): &Value, theme: &TailwindTheme) -> PluginResult<'static> {
    theme
        .colors
        .get(rest)
        .map(|c| {
            to_lit(&[
                ("--tw-gradient-from", c),
                ("--tw-gradient-to", c),
                (
                    "--tw-gradient-stops",
                    "var(--tw-gradient-from), var(--tw-gradient-to)",
                ),
            ])
        })
        .ok_or(vec![])
}

pub fn ring<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
    let rest = rest.map(|v| v.0).unwrap_or("DEFAULT");
    match rest.split_once('-') {
        Some(("offset", rest)) => {
            theme.ring_offset_width.get(rest)
                .map(|&s| ("--tw-ring-offset-width", s))
                .or_else(|| theme.colors.get(rest).map(|&s| ("--tw-ring-offset-color", s)))
                .map(|p| to_lit(&[p, ("boxShadow", "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)")]))
                .ok_or(vec![])
        }
        Some((_, _)) => ring_color(&Value(rest), theme),
        None if rest == "inset" => Ok(to_lit(&[("--tw-ring-inset", "inset")])),
        None => ring_base(&Value(rest), theme)
    }
}

pub fn flex<'a>(
    f: Option<Flex>,
    rest: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let rule = match (f, rest) {
        (Some(Flex::Row), _) => [("flexDirection", "row")],
        (Some(Flex::RowReverse), _) => [("flexDirection", "row-reverse")],
        (Some(Flex::Col), _) => [("flexDirection", "column")],
        (Some(Flex::ColReverse), _) => [("flexDirection", "column-reverse")],
        (Some(Flex::Wrap), _) => [("flexWrap", "wrap")],
        (Some(Flex::WrapReverse), _) => [("flexWrap", "wrap-reverse")],
        (Some(Flex::NoWrap), _) => [("flexWrap", "nowrap")],
        (None, None) => [("display", "flex")],
        (Some(Flex::Grow), val) => {
            return simple_lookup(
                &theme.flex_grow,
                val.as_ref().map(|v| v.as_str()).unwrap_or("DEFAULT"),
                "flexGrow",
            )
        }
        (Some(Flex::Shrink), val) => {
            return simple_lookup(
                &theme.flex_grow,
                val.as_ref().map(|v| v.as_str()).unwrap_or("DEFAULT"),
                "flexShrink",
            )
        }
        (None, Some(val)) => return simple_lookup(&theme.flex, val.as_str(), "flex"),
    };

    Ok(to_lit(&rule))
}

pub fn divide(
    d: Option<Divide>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> PluginResult<'static> {
    match (d, rest.as_ref().map(|r| r.as_str())) {
        (Some(Divide::X), Some("reverse")) => Ok(to_lit(&[("--tw-divide-x-reverse", "1")])),
        (Some(Divide::Y), Some("reverse")) => Ok(to_lit(&[("--tw-divide-y-reverse", "1")])),
        (Some(Divide::X), rest) => theme
            .divide_width
            .get(rest.unwrap_or("DEFAULT"))
            .map(|v| {
                to_lit(&[
                    ("--tw-divide-x-reverse", "0"),
                    (
                        "borderRightWidth",
                        &format!("calc({v} * var(--tw-divide-x-reverse))"),
                    ),
                    (
                        "borderLeftWidth",
                        &format!("calc({v} * calc(1 - var(--tw-divide-x-reverse)))"),
                    ),
                ])
            })
            .ok_or(vec![]),
        (Some(Divide::Y), rest) => theme
            .divide_width
            .get(rest.unwrap_or("DEFAULT"))
            .map(|v| {
                to_lit(&[
                    ("--tw-divide-y-reverse", "0"),
                    (
                        "borderTopWidth",
                        &format!("calc({v} * calc(1 - var(--tw-divide-y-reverse)))"),
                    ),
                    (
                        "borderBottomWidth",
                        &format!("calc({v} * var(--tw-divide-y-reverse))"),
                    ),
                ])
            })
            .ok_or(vec![]),
        (Some(Divide::None), None) => Ok(to_lit(&[("borderStyle", "none")])),
        (Some(Divide::Double), None) => Ok(to_lit(&[("borderStyle", "double")])),
        (Some(Divide::Dotted), None) => Ok(to_lit(&[("borderStyle", "dotted")])),
        (Some(Divide::Dashed), None) => Ok(to_lit(&[("borderStyle", "dashed")])),
        (Some(Divide::Solid), None) => Ok(to_lit(&[("borderStyle", "solid")])),
        (None, Some(rest)) => theme
            .colors
            .get(rest)
            .map(|v| to_lit(&[("borderColor", v)]))
            .ok_or(vec![]),
        _ => Err(vec![]),
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

pub fn align_self(
    a: AlignSelf,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> PluginResult<'static> {
    let rule = match a {
        AlignSelf::Auto => [("alignSelf", "auto")],
        AlignSelf::Start => [("alignSelf", "flex-start")],
        AlignSelf::End => [("alignSelf", "flex-end")],
        AlignSelf::Center => [("alignSelf", "center")],
        AlignSelf::Stretch => [("alignSelf", "stretch")],
        AlignSelf::Baseline => [("alignSelf", "baseline")],
    };

    Ok(to_lit(&rule))
}

pub fn placeholder<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
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

pub fn grid<'a>(
    g: Option<Grid>,
    rest: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let pair = match (g, rest) {
        (Some(Grid::Cols), Some(rest)) => {
            return grid_t_col(rest, theme);
        }
        (Some(Grid::Rows), Some(rest)) => return grid_t_row(rest, theme),
        (Some(Grid::FlowRow), None) => [("gridAutoFlow", "row")],
        (Some(Grid::FlowRowDense), None) => [("gridAutoFlow", "row dense")],
        (Some(Grid::FlowCol), None) => [("gridAutoFlow", "col")],
        (Some(Grid::FlowColDense), None) => [("gridAutoFlow", "col dense")],
        (Some(Grid::FlowDense), None) => [("gridAutoFlow", "dense")],
        (None, None) => [("display", "grid")],
        _ => return Err(vec![]),
    };
    Ok(to_lit(&pair))
}

pub fn object(
    o: Object,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> PluginResult<'static> {
    let rule = match o {
        Object::Contain => [("objectFit", "contain")],
        Object::Cover => [("objectFit", "cover")],
        Object::Fill => [("objectFit", "fill")],
        Object::None => [("objectFit", "none")],
        Object::ScaleDown => [("objectFit", "scale-down")],
    };
    Ok(to_lit(&rule))
}

pub fn white_space(
    o: Whitespace,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> PluginResult<'static> {
    let rule = match o {
        Whitespace::Normal => [("whiteSpace", "normal")],
        Whitespace::Nowrap => [("whiteSpace", "nowrap")],
        Whitespace::Pre => [("whiteSpace", "pre")],
        Whitespace::PreLine => [("whiteSpace", "pre-line")],
        Whitespace::PreWrap => [("whiteSpace", "pre-wrap")],
    };
    Ok(to_lit(&rule))
}

pub fn col<'a>(col: Option<Col>, value: &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match col {
        None => grid_col(value, theme),
        Some(Col::Start) => grid_col_start(value, theme),
        Some(Col::End) => grid_col_end(value, theme),
    }
}

pub fn row<'a>(row: Option<Row>, value: &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
    match row {
        None => grid_row(value, theme),
        Some(Row::Start) => grid_row_start(value, theme),
        Some(Row::End) => grid_row_end(value, theme),
    }
}

pub fn content<'a>(rest: &SubjectValue, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    Ok(to_lit(&[("content", rest.as_str())]))
}

pub fn justify<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    match *rest {
        "start" => Ok(("justifyContent", "flex-start")),
        "end" => Ok(("justifyContent", "flex-end")),
        "center" => Ok(("justifyContent", "center")),
        "between" => Ok(("justifyContent", "space-between")),
        "around" => Ok(("justifyContent", "space-around")),
        "evenly" => Ok(("justifyContent", "space-evenly")),
        "items-start" => Ok(("justifyItems", "start")),
        "items-end" => Ok(("justifyItems", "end")),
        "items-center" => Ok(("justifyItems", "center")),
        "items-stretch" => Ok(("justifyItems", "stretch")),
        "self-auto" => Ok(("justifySelf", "auto")),
        "self-start" => Ok(("justifySelf", "start")),
        "self-end" => Ok(("justifySelf", "end")),
        "self-center" => Ok(("justifySelf", "center")),
        "self-stretch" => Ok(("justifySelf", "stretch")),
        _ => Err(vec![]),
    }
    .map(|v| to_lit(&[v]))
}

pub fn italic() -> ObjectLit {
    to_lit(&[("fontStyle", "italic")])
}

pub fn transform<'a>(rest: Option<&Value>, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    Ok(to_lit(&[("transform", match rest {
            Some(Value("cpu")) | None => "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some(Value("gpu")) => "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some(Value("none")) => "none",
            _ => return Err(vec![]),
    })]))
}

pub fn display<'a>(
    d: Display,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    Ok(to_lit(&[(
        "display",
        match d {
            Display::Block => "block",
            Display::InlineBlock => "inline-block",
            Display::Inline => "inline",
            Display::InlineFlex => "inline-fled",
            Display::Table => "table",
            Display::InlineTable => "inline-table",
            Display::TableCaption => "table-caption",
            Display::TableCell => "table-cell",
            Display::TableColumn => "table-column",
            Display::TableColumnGroup => "table-column-group",
            Display::TableFooterGroup => "table-footer-group",
            Display::TableHeaderGroup => "table-header-group",
            Display::TableRowGroup => "table-row-group",
            Display::TableRow => "table-row",
            Display::FlowRoot => "flow-root",
            Display::InlineGrid => "inline-grid",
            Display::Contents => "contents",
            Display::ListItem => "list-item",
            Display::Hidden => "none",
        },
    )]))
}

pub fn line_clamp<'a>(rest: &SubjectValue, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    Ok(to_lit(&[
        ("overflow", "hidden"),
        ("display", "-webkit-box"),
        ("WebkitBoxOrient", "vertical"),
        ("WebkitLineClamp", rest.as_str()),
    ]))
}

pub fn overflow<'a>(
    o: Option<Overflow>,
    value: &Value,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    match o {
        Some(Overflow::X) => overflow_x(value, theme),
        Some(Overflow::Y) => overflow_y(value, theme),
        None => overflow_base(value, theme),
    }
}

pub fn position<'a>(
    p: Position,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    Ok(to_lit(&[(
        "position",
        match p {
            Position::Static => "static",
            Position::Fixed => "fixed",
            Position::Absolute => "absolute",
            Position::Relative => "relative",
            Position::Sticky => "sticky",
        },
    )]))
}

pub fn visibility<'a>(
    v: Visibility,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    Ok(to_lit(&[(
        "visibility",
        match v {
            Visibility::Visible => "visible",
            Visibility::Invisible => "hidden",
        },
    )]))
}

pub fn translate<'a>(
    t: Translate,
    val: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let val = match val {
        Some(v) => v,
        None => return Err(vec![]),
    };

    match t {
        Translate::X => translate_x(val, theme),
        Translate::Y => translate_y(val, theme),
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
pub fn sr<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
    "only"
        .eq(*rest)
        .then(|| {
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
        .ok_or(vec![])
}
