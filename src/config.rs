use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

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
    pub colors: HashMap<&'a str, &'a str>,
    #[serde(borrow, alias = "fontFamily")]
    pub font_family: HashMap<&'a str, Vec<Cow<'a, str>>>,
    #[serde(borrow, rename = "fontSize")]
    pub font_size: HashMap<&'a str, (&'a str, LineHeight<'a>)>,
    #[serde(borrow, rename = "fontWeight")]
    pub font_weight: HashMap<&'a str, &'a str>,
}

#[derive(Deserialize, Debug)]
pub struct LineHeight<'a> {
    #[serde(alias = "lineHeight")]
    pub line_height: &'a str,
}
