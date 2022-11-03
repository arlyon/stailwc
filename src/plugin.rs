use std::collections::HashMap;

use crate::{
    config::TailwindTheme,
    parse::nom::SubjectValue,
    util::{merge_literals, to_lit},
};
use itertools::Itertools;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str},
};
use tailwind_parse::{
    Border, Display, Divide, Flex, Grid, Object, Position, Rounded, TextDecoration, TextTransform,
    Visibility, Whitespace,
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

macro_rules! merge_plugins_opt {
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

macro_rules! merge_plugins {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def(rest: &str, theme: &TailwindTheme) -> Option<ObjectLit> {
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
lookup_plugin!(order, order, "order");
lookup_plugin!(bottom, height, "bottom");
lookup_plugin!(fill, colors, "fill");
lookup_plugin!(left, width, "left");
lookup_plugin!(right, width, "right");
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
lookup_plugin!(gap_x, gap, "columnGap");
lookup_plugin!(gap_y, gap, "rowGap");
lookup_plugin!(cursor, cursor, "cursor");
lookup_plugin!(scale, scale, "transform", |v| format!("scale({v})"));

lookup_plugin!(min_w, min_width, "minWidth");
lookup_plugin!(max_w, max_width, "maxWidth");
lookup_plugin!(min_h, min_height, "minHeight");
lookup_plugin!(max_h, max_height, "maxHeight");

lookup_plugin!(leading, line_height, "lineHeight");

pub fn rounded(
    subcommand: Option<Rounded>,
    rest: Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rest = match rest {
        Some(SubjectValue::Value(rest)) => rest,
        None => "DEFAULT",
        _ => return None,
    };

    let cmds = match subcommand {
        None => return simple_lookup(&theme.border_radius, rest, "borderRadius"),
        Some(Rounded::T) => ("borderTopLeftRadius", Some("borderTopRightRadius")),
        Some(Rounded::B) => ("borderBottomLeftRadius", Some("borderBottomRightRadius")),
        Some(Rounded::L) => ("borderTopLeftRadius", Some("borderBottomLeftRadius")),
        Some(Rounded::R) => ("borderTopRightRadius", Some("borderBottomRightRadius")),
        Some(Rounded::Tr) => ("borderTopRightRadius", None),
        Some(Rounded::Tl) => ("borderTopLeftRadius", None),
        Some(Rounded::Br) => ("borderBottomRightRadius", None),
        Some(Rounded::Bl) => ("borderBottomLeftRadius", None),
    };

    theme.border_radius.get(rest).map(|lookup| match cmds {
        (a, Some(b)) => to_lit(&[(a, lookup), (b, lookup)]),
        (a, None) => to_lit(&[(a, lookup)]),
    })
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
    _rest: Option<SubjectValue>,
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
    _rest: Option<SubjectValue>,
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

pub fn border(
    subcommand: Option<Border>,
    rest: Option<SubjectValue>,
    theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rest = match rest {
        Some(SubjectValue::Value(rest)) => rest,
        None => "DEFAULT",
        _ => return None,
    };

    let func = match subcommand {
        None => {
            return simple_lookup(&theme.colors, rest, "borderColor")
                .or_else(|| simple_lookup(&theme.border_width, rest, "borderWidth"))
        }
        Some(Border::X) => border_x,
        Some(Border::Y) => border_y,
        Some(Border::T) => border_t,
        Some(Border::B) => border_b,
        Some(Border::L) => border_l,
        Some(Border::R) => border_r,
    };

    func(Some(rest), theme)
}

lookup_plugin_opt!(border_t, border_width, "borderTopWidth");
lookup_plugin_opt!(border_l, border_width, "borderLeftWidth");
lookup_plugin_opt!(border_r, border_width, "borderRightWidth");
lookup_plugin_opt!(border_b, border_width, "borderBottomWidth");
merge_plugins_opt!(border_x, border_l, border_r);
merge_plugins_opt!(border_y, border_t, border_b);

merge_plugins!(inset_x, left, right);
merge_plugins!(inset_y, top, bottom);
merge_plugins!(inset, inset_x, inset_y);

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

pub fn flex(
    f: Option<Flex>,
    _rest: Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    let rule = match f {
        Some(Flex::Row) => [("flexDirection", "row")],
        Some(Flex::RowReverse) => [("flexDirection", "row-reverse")],
        Some(Flex::Col) => [("flexDirection", "column")],
        Some(Flex::ColReverse) => [("flexDirection", "column-reverse")],
        Some(Flex::Wrap) => [("flexWrap", "wrap")],
        Some(Flex::WrapReverse) => [("flexWrap", "wrap-reverse")],
        Some(Flex::NoWrap) => [("flexWrap", "nowrap")],
        None => [("display", "flex")],
    };

    Some(to_lit(&rule))
}

pub fn divide(
    d: Option<Divide>,
    rest: Option<SubjectValue>,
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

pub fn grid(
    g: Option<Grid>,
    rest: Option<SubjectValue>,
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

pub fn object(o: Object, _rest: Option<SubjectValue>, _theme: &TailwindTheme) -> Option<ObjectLit> {
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
    _rest: Option<SubjectValue>,
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

pub fn display(
    d: Display,
    _rest: Option<SubjectValue>,
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

pub fn position(
    p: Position,
    _rest: Option<SubjectValue>,
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
    _rest: Option<SubjectValue>,
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
