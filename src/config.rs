use std::{borrow::Cow, collections::HashMap};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig<'a> {
    #[serde(borrow)]
    pub config: TailwindConfig<'a>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TailwindConfig<'a> {
    #[serde(borrow)]
    pub theme: TailwindTheme<'a>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TailwindTheme<'a> {
    #[serde(borrow)]
    pub screens: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub spacing: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub cursor: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub flex: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "flexBasis")]
    pub flex_basis: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "flexGrow")]
    pub flex_grow: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "flexShrink")]
    pub flex_shrink: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub gap: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub scale: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub colors: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "fontFamily")]
    pub font_family: HashMap<&'a str, Vec<Cow<'a, str>>>,
    #[serde(borrow, rename = "fontSize")]
    pub font_size: HashMap<&'a str, (&'a str, LineHeight<'a>)>,
    #[serde(borrow, rename = "fontWeight")]
    pub font_weight: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "borderRadius")]
    pub border_radius: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "borderWidth")]
    pub border_width: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "boxShadow")]
    pub box_shadow: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "zIndex")]
    pub z_index: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub translate: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub width: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub height: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "letterSpacing")]
    pub letter_spacing: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub blur: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub invert: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "ringWidth")]
    pub ring_width: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "ringOffsetWidth")]
    pub ring_offset_width: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub opacity: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub order: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub margin: HashMap<&'a str, &'a str>,
    #[serde(borrow)]
    pub padding: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "outlineOffset")]
    pub outline_offset: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "outlineWidth")]
    pub outline_width: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "backgroundImage")]
    pub background_image: HashMap<&'a str, &'a str>,

    #[serde(borrow, rename = "gridTemplateRows")]
    pub grid_template_rows: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "gridTemplateColumns")]
    pub grid_template_columns: HashMap<&'a str, &'a str>,

    #[serde(borrow, rename = "transitionDelay")]
    pub transition_delay: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionDuration")]
    pub transition_duration: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionProperty")]
    pub transition_property: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionTimingFunction")]
    pub transition_timing_function: HashMap<&'a str, &'a str>,

    #[serde(borrow, alias = "minHeight")]
    pub min_height: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "minWidth")]
    pub min_width: HashMap<&'a str, &'a str>,
}

#[derive(Deserialize, Debug)]
pub struct LineHeight<'a> {
    #[serde(alias = "lineHeight")]
    pub line_height: &'a str,
}
