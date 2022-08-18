use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::{
    Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::infer::{infer_type, Type, SIZES};

pub fn parse_literal(s: &str) -> Result<ObjectLit, &str> {
    if let Some(pair) = s.split_once('-') {
        match pair {
            ("text", rest) => match infer_type(rest) {
                Ok(Type::Size(x)) => Ok(create_literal(
                    "fontSize",
                    &format!("{}em", SIZES.iter().position(|s| x.eq(*s)).unwrap()),
                )),
                Ok(Type::Color(x)) => Ok(create_literal("color", x)),
                _ => Err(s),
            },
            ("border", rest) => match infer_type(rest) {
                Ok(Type::Scalar(x)) => Ok(create_literal("borderWidth", &format!("{}px", x))),
                Ok(Type::Color(x)) => Ok(create_literal("borderColor", x)),
                _ => Err(s),
            },
            ("bg", rest) => Ok(create_literal("backgroundColor", rest)),
            ("h", rest) => Ok(create_literal("height", &format!("{}em", rest,))),
            ("w", rest) => Ok(create_literal("width", &format!("{}em", rest,))),
            ("p", rest) => Ok(create_literal("padding", &format!("{}em", rest,))),
            ("m", rest) => Ok(create_literal("margin", &format!("{}em", rest,))),
            _ => Err(s),
        }
    } else {
        Err(s)
    }
}

fn create_literal(key: &str, value: &str) -> ObjectLit {
    ObjectLit {
        span: DUMMY_SP,
        props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
                span: DUMMY_SP,
                raw: None,
                value: key.into(),
            }),
            value: Box::new(Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                raw: None,
                value: value.into(),
            }))),
        })))],
    }
}
