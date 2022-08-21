use std::collections::HashMap;

use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_visit::swc_ecma_ast::{
    Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

#[derive(Debug)]
enum KeyStrategy {
    Merge,
    Override,
}

/**
 * Inserts all the keys / values from b into a.
 * On clash:
 * - if both values are objects, merge
 * - otherwise, overwrite with b
 *
 * todo(arlyon): this could be slow for many keys
 */
pub fn merge_literals(mut a: ObjectLit, b: ObjectLit) -> ObjectLit {
    let mut strategies: HashMap<JsWord, (usize, KeyStrategy)> = Default::default();

    for (idx, prop) in a.props.iter().enumerate() {
        let (name, value) = match prop {
            PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                key:
                    PropName::Ident(Ident { sym: name, .. }) | PropName::Str(Str { value: name, .. }),
                value: box value,
            })) => (name, value),
            _ => continue,
        };

        strategies.insert(
            name.to_owned(),
            (
                idx,
                match value {
                    Expr::Object(_) => KeyStrategy::Merge,
                    _ => KeyStrategy::Override,
                },
            ),
        );
    }

    for prop in b.props {
        if let PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str { value: name, .. }) | PropName::Ident(Ident { sym: name, .. }),
            ..
        })) = &prop
        {
            match strategies.get(name) {
                Some((idx, KeyStrategy::Merge)) => {
                    if let (
                        Some(PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                            value: box Expr::Object(lit1),
                            ..
                        }))),
                        PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                            value: box Expr::Object(lit2),
                            ..
                        })),
                    ) = (a.props.get_mut(*idx), prop)
                    {
                        let temp = std::mem::replace(lit1, to_lit(&[]));
                        *lit1 = merge_literals(temp, lit2);
                    }
                }
                Some((idx, KeyStrategy::Override)) => a.props.insert(*idx, prop),
                _ => a.props.push(prop),
            }
        }
    }

    a
}

pub fn to_lit(items: &[(&str, &str)]) -> ObjectLit {
    ObjectLit {
        span: DUMMY_SP,
        props: items
            .iter()
            .map(|(key, value)| {
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident {
                        span: DUMMY_SP,
                        optional: false,
                        sym: (*key).into(),
                    }),
                    value: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        raw: None,
                        value: (*value).into(),
                    }))),
                })))
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use swc_common::DUMMY_SP;
    use swc_ecma_visit::swc_ecma_ast::{
        Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
    };

    use crate::util::{merge_literals, to_lit};

    #[test]
    fn a_and_b_merges() {
        let a = to_lit(&[("a", "value")]);
        let b = to_lit(&[("b", "value")]);
        let c = merge_literals(a, b);
        assert_eq!(c.props.len(), 2);
    }

    #[test]
    fn empty_b_does_nothing() {
        let a = ObjectLit {
            span: DUMMY_SP,
            props: vec![],
        };

        let b = ObjectLit {
            span: DUMMY_SP,
            props: vec![],
        };

        let c = merge_literals(a.clone(), b);

        assert_eq!(a, c);
    }

    #[test]
    fn override_b_replaces() {
        let a = to_lit(&[("replace", "a")]);
        let b = to_lit(&[("replace", "b")]);

        let c = merge_literals(a.clone(), b);

        if let PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
            value: box Expr::Lit(Lit::Str(Str { value, .. })),
            ..
        })) = &c.props[0]
        {
            assert_eq!("b", value.to_string())
        } else {
            panic!("fail")
        }
    }

    #[test]
    fn mergeable_b_merges() {
        let a = ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                value: Box::new(Expr::Object(to_lit(&[("a", "value")]))),
                key: PropName::Str(Str {
                    raw: None,
                    span: DUMMY_SP,
                    value: "merge".into(),
                }),
            })))],
        };
        let b = ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                value: Box::new(Expr::Object(to_lit(&[("b", "value")]))),
                key: PropName::Str(Str {
                    raw: None,
                    span: DUMMY_SP,
                    value: "merge".into(),
                }),
            })))],
        };

        let c = merge_literals(a.clone(), b);

        if let PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
            value: box Expr::Object(ObjectLit { props, .. }),
            ..
        })) = &c.props[0]
        {
            assert_eq!(props.len(), 2)
        } else {
            panic!("fail")
        }
    }
}
