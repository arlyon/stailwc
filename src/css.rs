use std::borrow::Cow;

pub fn format_css(forms: bool, fonts_sans: &[Cow<str>], fonts_mono: &[Cow<str>]) -> String {
    let reset = format!(
        include_str!("../data/reset.css"),
        fonts_sans.join(","),
        fonts_mono.join(",")
    );
    if forms {
        format!("{}{}", reset, include_str!("../data/form.css"))
    } else {
        reset
    }
}
