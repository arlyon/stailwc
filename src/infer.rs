use crate::config::TailwindTheme;

pub enum Type<'a> {
    Screen(&'a str),
    Scalar(u32),
    Color(&'a str),
}

pub fn infer_type<'a>(theme: &'a TailwindTheme, s: &'a str) -> Result<Type<'a>, &'a str> {
    if let Some(x) = theme.screens.get(s) {
        Ok(Type::Screen(x))
    } else if let Some(x) = theme.colors.get(s) {
        Ok(Type::Color(x))
    } else if let Ok(x) = s.parse::<u32>() {
        Ok(Type::Scalar(x))
    } else {
        Err(s)
    }
}
