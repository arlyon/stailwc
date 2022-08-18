pub enum Type<'a> {
    Size(&'a str),
    Scalar(u32),
    Color(&'a str),
}

pub static SIZES: [&str; 8] = ["xs", "s", "m", "lg", "xl", "2xl", "3xl", "4xl"];
static COLORS: [&str; 4] = ["red", "green", "blue", "yellow"];

pub fn infer_type(s: &str) -> Result<Type, &str> {
    match s {
        x if SIZES.contains(&x) => Ok(Type::Size(x)),
        x if COLORS.contains(&x) => Ok(Type::Color(x)),
        x => {
            if let Ok(x) = x.parse() {
                Ok(Type::Scalar(x))
            } else {
                Err(s)
            }
        }
    }
}
