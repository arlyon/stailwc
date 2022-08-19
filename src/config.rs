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
    #[serde(borrow, rename = "boxShadow")]
    pub box_shadow: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "zIndex")]
    pub z_index: HashMap<&'a str, &'a str>,

    #[serde(borrow, rename = "transitionDelay")]
    pub transition_delay: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionDuration")]
    pub transition_duration: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionProperty")]
    pub transition_property: HashMap<&'a str, &'a str>,
    #[serde(borrow, rename = "transitionTimingFunction")]
    pub transition_timing_function: HashMap<&'a str, &'a str>,
}

#[derive(Deserialize, Debug)]
pub struct LineHeight<'a> {
    #[serde(alias = "lineHeight")]
    pub line_height: &'a str,
}
