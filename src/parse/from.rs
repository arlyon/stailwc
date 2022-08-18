use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::Expr;
use swc_ecma_visit::swc_ecma_ast::KeyValueProp;
use swc_ecma_visit::swc_ecma_ast::Lit;
use swc_ecma_visit::swc_ecma_ast::ObjectLit;
use swc_ecma_visit::swc_ecma_ast::Prop;
use swc_ecma_visit::swc_ecma_ast::PropName;
use swc_ecma_visit::swc_ecma_ast::PropOrSpread;
use swc_ecma_visit::swc_ecma_ast::Str;

use crate::infer::infer_type;
use crate::infer::Type;
use crate::infer::SIZES;

use super::nom::Directive;
use super::nom::Expression;
use super::nom::Subject;

impl<'a> From<Directive<'a>> for ObjectLit {
    fn from(val: Directive<'a>) -> Self {
        val.exps
            .into_iter()
            .map(ObjectLit::from)
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

impl<'a> From<Expression<'a>> for ObjectLit {
    fn from(val: Expression<'a>) -> Self {
        let mut object: ObjectLit = match val.subject.try_into() {
            Ok(object) => object,
            Err(text) => {
                println!("fail : unknown subject `{}`", text);
                return ObjectLit {
                    span: DUMMY_SP,
                    props: vec![],
                };
            }
        };

        for prop in &mut object.props {
            if let PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                value: box Expr::Lit(Lit::Str(Str { value, .. })),
                ..
            })) = prop
            {
                *value = format!(
                    "{}{}{}",
                    if val.negative { "-" } else { "" },
                    value,
                    if val.important { " !important" } else { "" }
                )
                .into();
            }
        }

        for modifier in &val.modifiers {
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

            object = ObjectLit {
                span: DUMMY_SP,
                props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: value.into(),
                    }),
                    value: Box::new(Expr::Object(object)),
                })))],
            }
        }

        object
    }
}

impl<'a> TryFrom<Subject<'a>> for ObjectLit {
    type Error = &'a str;

    fn try_from(value: Subject<'a>) -> Result<Self, Self::Error> {
        match value {
            Subject::Literal(s) => {
                if let Some(pair) = s.split_once('-') {
                    match pair {
                        ("text", rest) => match infer_type(rest) {
                            Ok(Type::Size(x)) => Ok(create_lit(
                                "fontSize",
                                &format!("{}em", SIZES.iter().position(|s| x.eq(*s)).unwrap()),
                            )),
                            Ok(Type::Color(x)) => Ok(create_lit("color", x)),
                            _ => Err(s),
                        },
                        ("border", rest) => match infer_type(rest) {
                            Ok(Type::Scalar(x)) => {
                                Ok(create_lit("borderWidth", &format!("{}px", x)))
                            }
                            Ok(Type::Color(x)) => Ok(create_lit("borderColor", x)),
                            _ => Err(s),
                        },
                        ("bg", rest) => Ok(create_lit("backgroundColor", rest)),
                        ("h", rest) => Ok(create_lit("height", &format!("{}em", rest,))),
                        ("w", rest) => Ok(create_lit("width", &format!("{}em", rest,))),
                        ("p", rest) => Ok(create_lit("padding", &format!("{}em", rest,))),
                        ("m", rest) => Ok(create_lit("margin", &format!("{}em", rest,))),
                        _ => Err(s),
                    }
                } else {
                    Err(s)
                }
            }
            Subject::Group(dir) => Ok(dir.into()),
        }
    }
}

fn create_lit(key: &str, value: &str) -> ObjectLit {
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
