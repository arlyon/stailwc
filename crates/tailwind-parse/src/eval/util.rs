use std::{borrow::Cow, num::ParseIntError};

/// convert a hex color to the equivalent rgba() value
/// using the --tw-bg-opacity variable
pub fn convert_color<'a>(
    hex: &'a str,
    output_opacity: Option<&str>,
    alpha: Option<&str>,
) -> Result<Cow<'a, str>, ConversionError> {
    if let Some(rest) = hex.strip_prefix('#') {
        let (r, g, b) = if rest.len() == 6 {
            let r = u8::from_str_radix(&rest[0..2], 16)?;
            let g = u8::from_str_radix(&rest[2..4], 16)?;
            let b = u8::from_str_radix(&rest[4..6], 16)?;
            (r, g, b)
        } else if rest.len() == 3 {
            let r = u8::from_str_radix(&rest[0..1], 16)?;
            let g = u8::from_str_radix(&rest[1..2], 16)?;
            let b = u8::from_str_radix(&rest[2..3], 16)?;
            // return Ok(Cow::Owned(format!(
            //     "rgb({} {} {} / var({}))",
            //     r * 17,
            //     g * 17,
            //     b * 17,
            //     output_opacity
            // )));
            (r * 17, g * 17, b * 17)
        } else {
            return Err(ConversionError::InvalidLength);
        };

        match (output_opacity, alpha) {
            (Some(output_opacity), _) => Ok(Cow::Owned(format!(
                "rgb({} {} {} / var({}))",
                r, g, b, output_opacity
            ))),
            // alpha is a percentage, but needs to be a decimal
            (None, Some(alpha)) => {
                let alpha = alpha
                    .parse::<f32>()
                    .map_err(|_| ConversionError::InvalidAlpha)?
                    / 100.0;
                Ok(Cow::Owned(format!("rgb({} {} {} / {})", r, g, b, alpha)))
            }
            (None, None) => Ok(Cow::Borrowed(hex)),
        }
    } else {
        Ok(Cow::Borrowed(hex))
    }
}

#[derive(Debug)]
pub enum ConversionError {
    InvalidHex,
    InvalidLength,
    InvalidAlpha,
}

impl From<ParseIntError> for ConversionError {
    fn from(_: ParseIntError) -> Self {
        ConversionError::InvalidHex
    }
}

impl ConversionError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConversionError::InvalidHex => "invalid hex",
            ConversionError::InvalidLength => "invalid length",
            ConversionError::InvalidAlpha => "invalid alpha",
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case(
        "#abcdef",
        Some("--tw-bg-opacity"),
        None,
        "rgb(171 205 239 / var(--tw-bg-opacity))"
    )]
    #[test_case("#000", None, Some("50"), "rgb(0 0 0 / 0.5)")]
    fn test_convert_color(hex: &str, var: Option<&str>, alpha: Option<&str>, expected: &str) {
        assert_eq!(super::convert_color(hex, var, alpha).unwrap(), expected);
    }

    #[test_case("#zz" ; "wrong length")]
    #[test_case("#zzz" ; "out of range 3-length")]
    #[test_case("#zzzzzz" ; "out of range 6-length")]
    fn test_convert_color_bad(hex: &str) {
        let res = super::convert_color(hex, None, None);
        assert!(res.is_err(), "expected error: {:?}", res);
    }
}
