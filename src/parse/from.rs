use swc_core::common::Span;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::KeyValueProp;
use swc_core::ecma::ast::Lit;
use swc_core::ecma::ast::ObjectLit;
use swc_core::ecma::ast::Prop;
use swc_core::ecma::ast::PropName;
use swc_core::ecma::ast::PropOrSpread;
use swc_core::ecma::ast::Str;
use swc_core::plugin::errors::HANDLER;

use crate::config::Screens;
use crate::config::TailwindConfig;
use crate::util::merge_literals;

use super::literal::parse_literal;
use super::nom::Directive;
use super::nom::Expression;
use super::nom::Subject;

pub fn literal_from_directive<'a>(
    span: Span,
    val: Directive<'a>,
    config: &TailwindConfig,
) -> ObjectLit {
    val.exps
        .into_iter()
        .map(|e| literal_from_exp(span, e, config))
        .reduce(merge_literals)
        .unwrap_or_else(|| ObjectLit {
            span: DUMMY_SP,
            props: vec![],
        })
}

pub fn literal_from_exp<'a>(span: Span, val: Expression<'a>, config: &TailwindConfig) -> ObjectLit {
    let mut object: ObjectLit = match literal_from_subject(span, val.subject, config) {
        Ok(object) => object,
        Err(text) => {
            HANDLER.with(|handler| {
                handler
                    .struct_span_err(
                        val.span.unwrap_or(span),
                        &format!("unknown subject `{}`", text),
                    )
                    .emit()
            });
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
        let value = match config.theme.screens.get(modifier) {
            Some(Screens::Min(v)) => format!("@media (min-width: {})", v).into(),
            _ => match *modifier {
                "sm" => "@media (min-width: 640px)",
                "md" => "@media (min-width: 768px)",
                "lg" => "@media (min-width: 1024px)",
                "xl" => "@media (min-width: 1280px)",
                "2xl" => "@media (min-width: 1536px)",
                "before" => ":before",
                "after" => ":after",
                "hover" => ":hover",
                "first-letter" => "::first-letter",
                "marker" => "*::marker, ::marker",
                "selection" => "*::selection, ::selection",
                "file" => "::file-selector-button",
                "first-line" => "::first-line",
                "placeholder" => "::placeholder",
                "backdrop" => "::backdrop",
                "dark" => match config.dark_mode {
                    "media" => "@media (prefers-color-scheme: dark)",
                    "class" => ".dark",
                    _ => continue,
                },
                "light" => match config.dark_mode {
                    "media" => "@media (prefers-color-scheme: light)",
                    _ => continue,
                },
                "focus" => ":focus",
                "focus-within" => ":focus-within",
                "first" => ":first-child",
                "last" => ":last-child",
                "only" => ":only-child",
                "odd" => ":nth-child(odd)",
                "even" => ":nth-child(even)",
                "first-of-type" => ":first-of-type",
                "last-of-type" => ":last-of-type",
                "only-of-type" => ":only-of-type",
                "visited" => ":visited",
                "target" => ":target",
                "open" => "[open]",
                "default" => ":default",
                "checked" => ":checked",
                "indeterminate" => ":indeterminate",
                "placeholder-shown" => ":placeholder-shown",
                "autofill" => ":autofill",
                "optional" => ":optional",
                "required" => ":required",
                "valid" => ":valid",
                "invalid" => ":invalid",
                "in-range" => ":in-range",
                "out-of-range" => ":out-of-range",
                "read-only" => ":read-only",
                "empty" => ":empty",
                "enabled" => ":enabled",
                "focus-visible" => ":focus-visible",
                "active" => ":active",
                "disabled" => ":disabled",
                "all" => "*",
                "all-child" => "> *",
                "sibling" => "~ *",
                "hocus" => ":hover, :focus",
                "link" => ":link",
                "read-write" => ":read-write",
                "svg" => "svg",
                "even-of-type" => ":nth-of-type(even)",
                "odd-of-type" => ":nth-of-type(odd)",
                "ltr" => "[dir='ltr'] &",
                "rtl" => "[dir='rtl'] &",
                "motion-safe" => "@media (prefers-reduced-motion: no-preference)",
                "motion-reduce" => "@media (prefers-reduced-motion: reduce)",
                "print" => "@media print",
                "screen" => "@media screen",
                "portrait" => "@media portrait",
                "landscape" => "@media landscape",
                "group-hover" => ".group:hover &",
                x => {
                    HANDLER.with(|h| {
                        h.struct_span_err(span, &format!("unknown modifier `{}`", x))
                            .emit()
                    });
                    continue;
                }
            }
            .into(),
        };

        object = ObjectLit {
            span: DUMMY_SP,
            props: vec![Prop::KeyValue(KeyValueProp {
                key: PropName::Str(Str {
                    span: DUMMY_SP,
                    raw: None,
                    value,
                }),
                value: Box::new(Expr::Object(object)),
            })
            .into()],
        }
    }

    object
}

pub fn literal_from_subject<'a>(
    span: Span,
    value: Subject<'a>,
    config: &TailwindConfig,
) -> Result<ObjectLit, &'a str> {
    match value {
        Subject::Literal(lit) => parse_literal(&config.theme, lit),
        Subject::Group(dir) => Ok(literal_from_directive(span, dir, config)),
    }
}
