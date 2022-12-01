use std::collections::HashMap;

use crate::{
    AlignSelf, Border, Css, Display, Divide, Flex, Grid, Object, Position, Rounded, SubjectValue,
    TextDecoration, TextTransform, Value, Visibility, Whitespace,
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
        pub fn $def(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup(&theme.$map, rest, $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup_map(&theme.$map, rest, $target, $closure)
        }
    };
}

macro_rules! lookup_plugin_arbitrary {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def(value: &SubjectValue, theme: &TailwindTheme) -> Option<ObjectLit> {
            match value {
                SubjectValue::Value(Value(v)) => simple_lookup(&theme.$map, v, $target),
                SubjectValue::Css(Css(v)) => Some(to_lit(&[($target, v)])),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(value: &SubjectValue, theme: &TailwindTheme) -> Option<ObjectLit> {
            match value {
                SubjectValue::Value(Value(v)) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                SubjectValue::Css(Css(v)) => to_lit(&[($target, $closure(v))]),
            }
        }
    };
}

macro_rules! lookup_plugin_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
            simple_lookup(&theme.$map, rest.map(|v| v.0).unwrap_or("DEFAULT"), $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
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
        pub fn $def(value: Option<&SubjectValue>, theme: &TailwindTheme) -> Option<ObjectLit> {
            match value {
                Some(SubjectValue::Value(Value(v))) => simple_lookup(&theme.$map, v, $target),
                Some(SubjectValue::Css(Css(v))) => Some(to_lit(&[($target, v)])),
                // if there is no value, attempt to look up the default
                None => simple_lookup(&theme.$map, "DEFAULT", $target),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def(value: Option<&SubjectValue>, theme: &TailwindTheme) -> Option<ObjectLit> {
            match value {
                Some(SubjectValue::Value(Value(v))) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                Some(SubjectValue::Css(Css(v))) => to_lit(&[($target, $closure(v))]),
                // if there is no value, attempt to look up the default
                None => simple_lookup_map(&theme.$map, "DEFAULT", $target, $closure),
            }
        }
    };
}

macro_rules! merge_plugins {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def(rest: &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (None, None) => None,
                (None, Some(a)) => Some(a),
                (Some(b), None) => Some(b),
                (Some(a), Some(b)) => Some(merge_literals(a, b)),
            }
        }
    };
}

macro_rules! merge_plugins_opt {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
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
        pub fn $def(rest: Option<&SubjectValue>, theme: &TailwindTheme) -> Option<ObjectLit> {
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
lookup_plugin_opt!(blur, blur, "filter", |s| format!("blur({s})"));
lookup_plugin_opt!(invert, invert, "filter", |s| format!("invert({s})"));
lookup_plugin!(basis, flex_basis, "flexBasis");
lookup_plugin_opt!(grow, flex_grow, "flexGrow");
lookup_plugin_opt!(shrink, flex_shrink, "flexShrink");
lookup_plugin!(top, height, "top");
lookup_plugin!(opacity, opacity, "opacity");
lookup_plugin!(animation, animation, "animation");
lookup_plugin!(order, order, "order");
lookup_plugin!(bottom, height, "bottom");
lookup_plugin!(fill, colors, "fill");
lookup_plugin!(left, width, "left");
lookup_plugin!(right, width, "right");
lookup_plugin_arbitrary!(tracking, letter_spacing, "letterSpacing");
lookup_plugin_arbitrary!(h, height, "height");
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
lookup_plugin!(gap, gap, "gap");
lookup_plugin!(gap_x, gap, "columnGap");
lookup_plugin!(gap_y, gap, "rowGap");
lookup_plugin!(cursor, cursor, "cursor");
lookup_plugin!(scale, scale, "transform", |v| format!("scale({v})"));

lookup_plugin_arbitrary!(min_w, min_width, "minWidth");
lookup_plugin_arbitrary!(max_w, max_width, "maxWidth");
lookup_plugin_arbitrary!(min_h, min_height, "minHeight");
lookup_plugin_arbitrary!(max_h, max_height, "maxHeight");

lookup_plugin!(leading, line_height, "lineHeight");

pub fn align(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match *rest {
        "baseline" => Some(to_lit(&[("verticalAlign", "baseline")])),
        "top" => Some(to_lit(&[("verticalAlign", "top")])),
        "middle" => Some(to_lit(&[("verticalAlign", "middle")])),
        "bottom" => Some(to_lit(&[("verticalAlign", "bottom")])),
        "text-top" => Some(to_lit(&[("verticalAlign", "text-top")])),
        "text-bottom" => Some(to_lit(&[("verticalAlign", "text-bottom")])),
        "sub" => Some(to_lit(&[("verticalAlign", "sub")])),
        "super" => Some(to_lit(&[("verticalAlign", "super")])),
        _ => None,
    }
}

pub fn rounded(
    subcommand: Option<Rounded>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let cmds = match subcommand {
        None => {
            return match rest {
                Some(SubjectValue::Value(Value(v))) => {
                    simple_lookup(&theme.border_radius, v, "borderRadius")
                }
                Some(SubjectValue::Css(Css(v))) => {
                    Some(to_lit(&[("borderRadius", &v.to_string())]))
                }
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

    radius.map(|lookup| match cmds {
        (a, Some(b)) => to_lit(&[(a, lookup), (b, lookup)]),
        (a, None) => to_lit(&[(a, lookup)]),
    })
}

pub fn pointer_events(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match *rest {
        "events-none" => Some(to_lit(&[("pointerEvents", "none")])),
        "events-auto" => Some(to_lit(&[("pointerEvents", "auto")])),
        _ => None,
    }
}

pub fn mix(rest: &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest.0.split_once('-') {
        Some(("blend", rest)) => blend(rest, theme),
        _ => None,
    }
}

pub fn transform_origin(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match *rest {
        "center" => Some(to_lit(&[("transformOrigin", "center")])),
        "top" => Some(to_lit(&[("transformOrigin", "top")])),
        "top-right" => Some(to_lit(&[("transformOrigin", "top right")])),
        "right" => Some(to_lit(&[("transformOrigin", "right")])),
        "bottom-right" => Some(to_lit(&[("transformOrigin", "bottom right")])),
        "bottom" => Some(to_lit(&[("transformOrigin", "bottom")])),
        "bottom-left" => Some(to_lit(&[("transformOrigin", "bottom left")])),
        "left" => Some(to_lit(&[("transformOrigin", "left")])),
        "top-left" => Some(to_lit(&[("transformOrigin", "top left")])),
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

pub fn text(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_size, rest, "fontSize", |(a, _)| a.to_string())
        .or_else(|| simple_lookup(&theme.colors, rest, "color"))
        .or_else(|| {
            ["left", "center", "right", "justify", "start", "end"]
                .contains(&rest)
                .then_some(to_lit(&[("textAlign", rest)]))
        })
}

pub fn space(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
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
                    &format!("calc({v} * var(--tw-space-x-reverse))"),
                ),
                (
                    "marginLeft",
                    &format!("calc({v} * calc(1 - var(--tw-space-x-reverse)))"),
                ),
            ])
        }),
        Some(("y", rest)) => theme.space.get(rest).map(|v| {
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

pub fn text_transform(
    tt: TextTransform,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "textTransform",
        match tt {
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
            TextTransform::NormalCase => "none",
        },
    )]))
}

pub fn text_decoration(
    td: TextDecoration,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "textDecorationLine",
        match td {
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::NoUnderline => "none",
        },
    )]))
}

pub fn truncate(_rest: Option<&Value>, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[
        ("overflow", "hidden"),
        ("textOverflow", "ellipsis"),
        ("whiteSpace", "nowrap"),
    ]))
}

pub fn appearance(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    "none"
        .eq(*rest)
        .then_some(to_lit(&[("appearance", "none")]))
}

pub fn font(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup_map(&theme.font_family, rest, "fontFamily", |s| {
        s.iter().join(", ")
    })
    .or_else(|| simple_lookup(&theme.font_weight, rest, "fontWeight"))
}

pub fn outline(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        None => Some(to_lit(&[("outlineStyle", "solid")])),
        Some(Value("none")) => Some(to_lit(&[
            ("outline", "2px solid transparent"),
            ("outlineOffset", "2px"),
        ])),
        Some(Value("dashed")) => Some(to_lit(&[("outlineStyle", "dashed")])),
        Some(Value("dotted")) => Some(to_lit(&[("outlineStyle", "dotted")])),
        Some(Value("double")) => Some(to_lit(&[("outlineStyle", "double")])),
        Some(Value("hidden")) => Some(to_lit(&[("outlineStyle", "hidden")])),
        Some(rest) => simple_lookup(&theme.colors, rest.0, "outlineColor")
            .or_else(|| simple_lookup(&theme.outline_offset, rest.0, "outlineOffset"))
            .or_else(|| simple_lookup(&theme.outline_width, rest.0, "outlineWidth")),
    }
}

pub fn bg(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.colors, rest, "backgroundColor")
        .or_else(|| simple_lookup(&theme.background_image, rest, "backgroundImage"))
}

pub fn shadow(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
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
}

pub fn border(
    subcommand: Option<Border>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
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

fn border_style(rest: Option<&SubjectValue>, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match rest {
        Some(SubjectValue::Value(Value("solid"))) => Some(to_lit(&[("borderStyle", "solid")])),
        Some(SubjectValue::Value(Value("dashed"))) => Some(to_lit(&[("borderStyle", "dashed")])),
        Some(SubjectValue::Value(Value("dotted"))) => Some(to_lit(&[("borderStyle", "dotted")])),
        Some(SubjectValue::Value(Value("double"))) => Some(to_lit(&[("borderStyle", "double")])),
        Some(SubjectValue::Value(Value("hidden"))) => Some(to_lit(&[("borderStyle", "hidden")])),
        Some(SubjectValue::Value(Value("none"))) => Some(to_lit(&[("borderStyle", "none")])),
        _ => None,
    }
}

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

merge_plugins!(inset_x, left, right);
merge_plugins!(inset_y, top, bottom);
merge_plugins!(inset, inset_x, inset_y);

pub fn from(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
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

pub fn ring(rest: Option<&Value>, theme: &TailwindTheme) -> Option<ObjectLit> {
    let rest = rest.map(|v| v.0).unwrap_or("DEFAULT");
    match rest.split_once('-') {
        Some(("offset", rest)) => {
            theme.ring_offset_width.get(rest)
                .map(|&s| ("--tw-ring-offset-width", s))
                .or_else(|| theme.colors.get(rest).map(|&s| ("--tw-ring-offset-color", s)))
                .map(|p| to_lit(&[p, ("boxShadow", "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow)")]))
        }
        Some((_, _)) => simple_lookup(&theme.colors, rest, "--tw-ring-color"),
        None => (rest == "inset").then(|| to_lit(&[("--tw-ring-inset", "inset")])).or_else(||simple_lookup(&theme.ring_width, rest, "borderWidth")).or_else(|| simple_lookup(&theme.colors, rest, "--tw-ring-color")),
    }
}

pub fn flex(
    f: Option<Flex>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
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

    Some(to_lit(&rule))
}

pub fn divide(
    d: Option<Divide>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
    match (d, rest.as_ref().map(|r| r.as_str())) {
        (Some(Divide::X), Some("reverse")) => Some(to_lit(&[("--tw-divide-x-reverse", "1")])),
        (Some(Divide::Y), Some("reverse")) => Some(to_lit(&[("--tw-divide-y-reverse", "1")])),
        (Some(Divide::X), rest) => theme.divide_width.get(rest.unwrap_or("DEFAULT")).map(|v| {
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
        }),
        (Some(Divide::Y), rest) => theme.divide_width.get(rest.unwrap_or("DEFAULT")).map(|v| {
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
        }),
        (Some(Divide::None), None) => Some(to_lit(&[("borderStyle", "none")])),
        (Some(Divide::Double), None) => Some(to_lit(&[("borderStyle", "double")])),
        (Some(Divide::Dotted), None) => Some(to_lit(&[("borderStyle", "dotted")])),
        (Some(Divide::Dashed), None) => Some(to_lit(&[("borderStyle", "dashed")])),
        (Some(Divide::Solid), None) => Some(to_lit(&[("borderStyle", "solid")])),
        (None, Some(rest)) => theme
            .colors
            .get(rest)
            .map(|v| to_lit(&[("borderColor", v)])),
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

pub fn align_self(
    a: AlignSelf,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rule = match a {
        AlignSelf::Auto => [("alignSelf", "auto")],
        AlignSelf::Start => [("alignSelf", "flex-start")],
        AlignSelf::End => [("alignSelf", "flex-end")],
        AlignSelf::Center => [("alignSelf", "center")],
        AlignSelf::Stretch => [("alignSelf", "stretch")],
        AlignSelf::Baseline => [("alignSelf", "baseline")],
    };

    Some(to_lit(&rule))
}

pub fn placeholder(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
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

pub fn grid(
    g: Option<Grid>,
    rest: &Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rest = rest.as_ref().map(|s| s.as_str());
    let pair = match (g, rest) {
        (Some(Grid::Cols), Some(rest)) => {
            return simple_lookup(&theme.grid_template_columns, rest, "gridTemplateColumns")
        }
        (Some(Grid::Rows), Some(rest)) => {
            return simple_lookup(&theme.grid_template_rows, rest, "gridTemplateRows")
        }
        (Some(Grid::FlowRow), None) => [("gridAutoFlow", "row")],
        (Some(Grid::FlowRowDense), None) => [("gridAutoFlow", "row dense")],
        (Some(Grid::FlowCol), None) => [("gridAutoFlow", "col")],
        (Some(Grid::FlowColDense), None) => [("gridAutoFlow", "col dense")],
        (Some(Grid::FlowDense), None) => [("gridAutoFlow", "dense")],
        (None, None) => [("display", "grid")],
        _ => return None,
    };
    Some(to_lit(&pair))
}

pub fn object(
    o: Object,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rule = match o {
        Object::Contain => [("objectFit", "contain")],
        Object::Cover => [("objectFit", "cover")],
        Object::Fill => [("objectFit", "fill")],
        Object::None => [("objectFit", "none")],
        Object::ScaleDown => [("objectFit", "scale-down")],
    };
    Some(to_lit(&rule))
}

pub fn white_space(
    o: Whitespace,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rule = match o {
        Whitespace::Normal => [("whiteSpace", "normal")],
        Whitespace::Nowrap => [("whiteSpace", "nowrap")],
        Whitespace::Pre => [("whiteSpace", "pre")],
        Whitespace::PreLine => [("whiteSpace", "pre-line")],
        Whitespace::PreWrap => [("whiteSpace", "pre-wrap")],
    };
    Some(to_lit(&rule))
}

pub fn col(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.grid_column, rest, "gridColumn").or_else(|| match rest.split_once('-') {
        Some(("start", rest)) => simple_lookup(&theme.grid_column_start, rest, "gridColumnStart"),
        _ => None,
    })
}

pub fn row(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    simple_lookup(&theme.grid_row, rest, "gridRow").or_else(|| match rest.split_once('-') {
        Some(("start", rest)) => simple_lookup(&theme.grid_row_start, rest, "gridRowStart"),
        Some(("end", rest)) => simple_lookup(&theme.grid_row_end, rest, "gridRowEnd"),
        _ => None,
    })
}

pub fn justify(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match *rest {
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

pub fn italic() -> Option<ObjectLit> {
    Some(to_lit(&[("fontStyle", "italic")]))
}

pub fn items(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    match *rest {
        "start" => Some("flex-start"),
        "end" => Some("flex-end"),
        "center" => Some("center"),
        "baseline" => Some("baseline"),
        "stretch" => Some("stretch"),
        _ => None,
    }
    .map(|v| to_lit(&[("alignItems", v)]))
}

pub fn transform(rest: Option<&Value>, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[("transform", match rest {
            Some(Value("cpu")) | None => "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some(Value("gpu")) => "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
            Some(Value("none")) => "none",
            _ => return None,
    })]))
}

pub fn display(
    d: Display,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    Some(to_lit(&[(
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

pub fn box_(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "boxSizing",
        match *rest {
            "border" => "border-box",
            "content" => "content-box",
            _ => return None,
        },
    )]))
}

pub fn select(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    ["none", "text", "all", "auto"]
        .contains(&rest)
        .then_some(to_lit(&[("userSelect", rest)]))
}

pub fn overflow(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
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

pub fn position(
    p: Position,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    Some(to_lit(&[(
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

pub fn visibility(
    v: Visibility,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    Some(to_lit(&[(
        "visibility",
        match v {
            Visibility::Visible => "visible",
            Visibility::Invisible => "hidden",
        },
    )]))
}

pub fn translate(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
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
pub fn sr(Value(rest): &Value, _theme: &TailwindTheme) -> Option<ObjectLit> {
    "only".eq(*rest).then(|| {
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

pub fn px(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .padding
        .get(rest)
        .map(|s| to_lit(&[("paddingLeft", s), ("paddingRight", s)]))
}

pub fn py(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .padding
        .get(rest)
        .map(|s| to_lit(&[("paddingTop", s), ("paddingBottom", s)]))
}

pub fn mx(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .margin
        .get(rest)
        .map(|s| to_lit(&[("marginLeft", s), ("marginRight", s)]))
}

pub fn my(Value(rest): &Value, theme: &TailwindTheme) -> Option<ObjectLit> {
    theme
        .margin
        .get(rest)
        .map(|s| to_lit(&[("marginTop", s), ("marginBottom", s)]))
}
