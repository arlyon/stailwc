use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::Expr;
use swc_ecma_visit::swc_ecma_ast::KeyValueProp;
use swc_ecma_visit::swc_ecma_ast::Lit;
use swc_ecma_visit::swc_ecma_ast::ObjectLit;
use swc_ecma_visit::swc_ecma_ast::Prop;
use swc_ecma_visit::swc_ecma_ast::PropName;
use swc_ecma_visit::swc_ecma_ast::PropOrSpread;
use swc_ecma_visit::swc_ecma_ast::Str;

use crate::config::TailwindTheme;
use crate::util::merge_literals;

use super::literal::parse_literal;
use super::nom::Directive;
use super::nom::Expression;
use super::nom::Subject;

pub fn literal_from_directive<'a>(val: Directive<'a>, theme: &TailwindTheme) -> ObjectLit {
    val.exps
        .into_iter()
        .map(|e| literal_from_exp(e, theme))
        .reduce(merge_literals)
        .unwrap_or_else(|| ObjectLit {
            span: DUMMY_SP,
            props: vec![],
        })
}

pub fn literal_from_exp<'a>(val: Expression<'a>, theme: &TailwindTheme) -> ObjectLit {
    let mut object: ObjectLit = match literal_from_subject(val.subject, theme) {
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
            "focus-within" => "&:focus-within",
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

pub fn literal_from_subject<'a>(
    value: Subject<'a>,
    theme: &TailwindTheme,
) -> Result<ObjectLit, &'a str> {
    match value {
        Subject::Literal(s) => parse_literal(theme, s),
        Subject::Group(dir) => Ok(literal_from_directive(dir, theme)),
    }
}
