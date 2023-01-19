use std::collections::HashMap;

use crate::{
    AlignSelf, Backdrop, Border, Col, Css, Display, Divide, Flex, Grid, Object, Overflow,
    PluginResult, Position, Rounded, Row, SubjectValue, TextDecoration, TextTransform, Translate,
    Value, Visibility, Whitespace,
};
use itertools::Itertools;
use stailwc_swc_utils::{merge_literals, to_lit};
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};
use tailwind_config::TailwindTheme;

use super::macros::*;

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

lookup_plugin_arbitrary!(stroke_width, stroke_width, "strokeWidth");
lookup_plugin_arbitrary!(stroke_color, colors, "stroke");
merge_plugins_arbitrary!(stroke, stroke_width, stroke_color);

lookup_plugin_opt!(transition, transition_property, "transitionProperty");
lookup_plugin_arbitrary!(delay, transition_delay, "transitionDelay");
lookup_plugin_opt!(duration, transition_duration, "transitionDuration");
lookup_plugin_opt!(ease, transition_timing_function, "transitionTimingFunction");
lookup_plugin_opt!(blur, blur, "filter", |s| format!("blur({s})"));
lookup_plugin_opt!(invert, invert, "filter", |s| format!("invert({s})"));
lookup_plugin!(basis, flex_basis, "flexBasis");
lookup_plugin_opt!(grow, flex_grow, "flexGrow");
lookup_plugin_opt!(shrink, flex_shrink, "flexShrink");
lookup_plugin_arbitrary!(top, height, "top");
array_map_plugin!(
    auto_rows,
    [
        ("auto", "auto"),
        ("min", "min-content"),
        ("max", "max-content"),
        ("fr", "minmax(0, 1fr)")
    ],
    "gridAutoRows"
);
array_map_plugin!(
    auto_cols,
    [
        ("auto", "auto"),
        ("min", "min-content"),
        ("max", "max-content"),
        ("fr", "minmax(0, 1fr)")
    ],
    "gridAutoColumns"
);
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
lookup_plugin_arbitrary!(p, padding, "padding");
lookup_plugin_arbitrary!(pl, padding, "paddingLeft");
lookup_plugin_arbitrary!(pr, padding, "paddingRight");
lookup_plugin_arbitrary!(pt, padding, "paddingTop");
lookup_plugin_arbitrary!(pb, padding, "paddingBottom");

lookup_plugin_arbitrary!(m, margin, "margin");
lookup_plugin_arbitrary!(ml, margin, "marginLeft");
lookup_plugin_arbitrary!(mr, margin, "marginRight");
lookup_plugin_arbitrary!(mt, margin, "marginTop");
lookup_plugin_arbitrary!(mb, margin, "marginBottom");
lookup_plugin_arbitrary!(z, z_index, "zIndex");
lookup_plugin_arbitrary!(gap, gap, "gap");
lookup_plugin_arbitrary!(gap_x, gap, "columnGap");
lookup_plugin_arbitrary!(gap_y, gap, "rowGap");
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
merge_plugins_arbitrary!(text_base, text_color, text_size);

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
merge_plugins_arbitrary_opt!(border_inner, border_cw, border_style_wrapped);

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

merge_plugins_arbitrary!(my, mt, mb);
merge_plugins_arbitrary!(mx, ml, mr);
merge_plugins_arbitrary!(py, pt, pb);
merge_plugins_arbitrary!(px, pl, pr);

lookup_plugin_arbitrary_opt!(rounded_base, border_radius, "borderRadius");
lookup_plugin_arbitrary_opt!(rounded_tl, border_radius, "borderTopLeftRadius");
lookup_plugin_arbitrary_opt!(rounded_tr, border_radius, "borderTopRightRadius");
lookup_plugin_arbitrary_opt!(rounded_bl, border_radius, "borderBottomLeftRadius");
lookup_plugin_arbitrary_opt!(rounded_br, border_radius, "borderBottomRightRadius");
merge_plugins_arbitrary_opt!(rounded_t, rounded_tl, rounded_tr);
merge_plugins_arbitrary_opt!(rounded_b, rounded_bl, rounded_br);
merge_plugins_arbitrary_opt!(rounded_l, rounded_tl, rounded_bl);
merge_plugins_arbitrary_opt!(rounded_r, rounded_tr, rounded_br);

pub fn rounded<'a>(
    subcommand: Option<Rounded>,
    rest: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    let fun = match subcommand {
        None => rounded_base,
        Some(Rounded::T) => rounded_t,
        Some(Rounded::B) => rounded_b,
        Some(Rounded::L) => rounded_l,
        Some(Rounded::R) => rounded_r,
        Some(Rounded::Tr) => rounded_tr,
        Some(Rounded::Tl) => rounded_tl,
        Some(Rounded::Br) => rounded_br,
        Some(Rounded::Bl) => rounded_bl,
    };

    fun(rest.as_ref(), theme)
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
    text_base(value, theme).or_else(|e| match value {
        SubjectValue::Value(v) => {
            text_align(v, theme).map_err(|e2| e.into_iter().chain(e2).collect())
        }
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

lookup_plugin_arbitrary!(aspect, aspect_ratio, "aspectRatio");

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

array_plugin!(
    border_style,
    ["solid", "dashed", "dotted", "double", "hidden", "none"],
    "borderStyle"
);

/// wrap border_style to be composed with other plugins
fn border_style_wrapped<'a>(
    rest: Option<&SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    match rest {
        Some(SubjectValue::Value(val)) => border_style(val, theme),
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

lookup_plugin_opt!(flex_grow, flex_grow, "flexGrow");
lookup_plugin_opt!(flex_shrink, flex_shrink, "flexShrink");

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
        (Some(Flex::Grow), v) => return flex_grow(v.as_ref().and_then(|s| s.value()), theme),
        (Some(Flex::Shrink), v) => return flex_shrink(v.as_ref().and_then(|s| s.value()), theme),
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

lookup_plugin_arbitrary_opt!(backdrop_blur, backdrop_blur, "backdropBlur", |s| format!(
    "blur({})",
    s
));

pub fn backdrop<'a>(
    o: Backdrop,
    value: &Option<SubjectValue>,
    theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    match o {
        Backdrop::Blur => backdrop_blur(value.as_ref(), theme),
        Backdrop::Brightness => todo!(),
        Backdrop::Contrast => todo!(),
        Backdrop::Grayscale => todo!(),
        Backdrop::HueRotate => todo!(),
        Backdrop::Invert => todo!(),
        Backdrop::Opacity => todo!(),
        Backdrop::Saturate => todo!(),
        Backdrop::Sepia => todo!(),
    }
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

array_map_plugin!(
    justify_content,
    [
        ("start", "flex-start"),
        ("end", "flex-end"),
        ("center", "center"),
        ("between", "space-between"),
        ("around", "space-around"),
        ("evenly", "space-evenly")
    ],
    "justifyContent"
);

array_map_plugin!(
    justify_items,
    [
        ("items-start", "start"),
        ("items-end", "end"),
        ("items-center", "center"),
        ("iterms-stretch", "stretch")
    ],
    "justifyItems"
);

array_map_plugin!(
    justify_self,
    [
        ("self-auto", "auto"),
        ("self-start", "start"),
        ("self-end", "end"),
        ("self-center", "center"),
        ("self-stretch", "stretch")
    ],
    "justifySelf"
);

merge_plugins!(justify_content_items, justify_content, justify_items);
merge_plugins!(justify, justify_content_items, justify_self);

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
