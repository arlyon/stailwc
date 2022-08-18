use std::str::FromStr;

use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::{
    Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::infer::{infer_type, Type, SIZES};

pub struct Directive<'a>(Vec<Expression<'a>>);

impl<'a> From<&'a str> for Directive<'a> {
    fn from(s: &'a str) -> Self {
        Directive(s.split_ascii_whitespace().map(Expression::from).collect())
    }
}

impl<'a> Directive<'a> {
    pub fn parse(&self) -> ObjectLit {
        self.0
            .iter()
            .map(Expression::parse)
            .reduce(|mut acc, mut curr| {
                acc.props.append(&mut curr.props);
                acc
            })
            .unwrap_or_else(|| ObjectLit {
                span: DUMMY_SP,
                props: vec![],
            })
    }
}

#[derive(Debug)]
struct Expression<'a> {
    negative: bool,
    modifiers: Vec<&'a str>,
    subject: &'a str,
    alpha: Option<&'a str>,
    important: bool,
}

impl<'a> From<&'a str> for Expression<'a> {
    fn from(s: &'a str) -> Self {
        let (negative, s) = if s.starts_with('-') {
            (true, &s[1..])
        } else {
            (false, s)
        };

        let mut modifiers: Vec<_> = s.split(":").collect();
        let s = modifiers.pop().unwrap();

        let (important, s) = if s.ends_with("!") {
            (true, &s[..s.len() - 1])
        } else {
            (false, s)
        };

        let (s, alpha) = match s.split_once("/") {
            Some((a, b)) => (a, Some(b)),
            None => (s, None),
        };

        Expression {
            negative,
            modifiers,
            subject: s,
            alpha,
            important,
        }
    }
}

impl<'a> Expression<'a> {
    pub fn parse(&self) -> ObjectLit {
        // generate item

        let prop = if let Ok((k, v)) = parse_subject(self.subject) {
            let v = format!(
                "{}{}{}",
                if self.negative { "-" } else { "" },
                v,
                if self.important { " !important" } else { "" }
            );

            let mut prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Str(Str {
                    span: DUMMY_SP,
                    raw: None,
                    value: k.into(),
                }),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    raw: None,
                    value: v.into(),
                }))),
            })));

            for modifier in &self.modifiers {
                let value = match *modifier {
                    "sm" => "@media(min-width: 640px)",
                    "md" => "@media(min-width: 768px)",
                    "lg" => "@media(min-width: 1024px)",
                    "xl" => "@media(min-width: 1280x)",
                    "2xl" => "@media(min-width: 1536x)",
                    "hover" => "&:hover",
                    "focus" => "&:focus",
                    _ => continue,
                };

                prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: value.into(),
                    }),
                    value: Box::new(Expr::Object(ObjectLit {
                        span: DUMMY_SP,
                        props: vec![prop],
                    })),
                })))
            }

            prop
        } else {
            println!("fail : unknown subject `{}`", self.subject);
            return ObjectLit {
                span: DUMMY_SP,
                props: vec![],
            };
        };

        println!("{:#?}", prop);

        ObjectLit {
            span: DUMMY_SP,
            props: vec![prop],
        }
    }
}

fn parse_subject(s: &str) -> Result<(&str, String), &str> {
    let exp: Expression = s.into();

    if let Some(pair) = exp.subject.split_once('-') {
        match pair {
            ("text", rest) => match infer_type(rest) {
                Ok(Type::Size(x)) => Ok((
                    "fontSize",
                    format!("{}em", SIZES.iter().position(|s| x.eq(*s)).unwrap()),
                )),
                Ok(Type::Color(x)) => Ok(("color", x.to_string())),
                _ => Err(s),
            },
            ("border", rest) => match infer_type(rest) {
                Ok(Type::Scalar(x)) => Ok(("borderWidth", format!("{}px", x))),
                Ok(Type::Color(x)) => Ok(("borderColor", x.to_string())),
                _ => Err(s),
            },
            ("bg", rest) => Ok(("backgroundColor", rest.to_string())),
            ("h", rest) => Ok(("height", format!("{}em", rest,))),
            ("w", rest) => Ok(("width", format!("{}em", rest,))),
            ("p", rest) => Ok(("padding", format!("{}em", rest,))),
            ("m", rest) => Ok(("margin", format!("{}em", rest,))),
            _ => Err(s),
        }
    } else {
        Err(s)
    }
}
