macro_rules! lookup_plugin {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup(&theme.$map, rest, $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(Value(rest): &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup_map(&theme.$map, rest, $target, $closure)
        }
    };
}

macro_rules! array_plugin {
    ($def:ident, $options:expr, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
            $options
                .iter()
                .find(|&x| x == rest)
                .map(|_| to_lit(&[($target, rest)]))
                .ok_or_else(|| {
                    let sort = eddie::Levenshtein::new();
                    $options
                        .iter()
                        .sorted_by_key(|val| sort.distance(rest, val))
                        .copied()
                        .take(5)
                        .collect()
                })
        }
    };
}

macro_rules! array_map_plugin {
    ($def:ident, $options:expr, $target:expr) => {
        pub fn $def<'a>(Value(rest): &Value, _theme: &'a TailwindTheme) -> PluginResult<'a> {
            $options
                .iter()
                .find(|(x, _)| x == rest)
                .map(|(_, y)| to_lit(&[($target, y)]))
                .ok_or_else(|| {
                    let sort = eddie::Levenshtein::new();
                    $options
                        .iter()
                        .map(|(x, _)| x)
                        .sorted_by_key(|val| sort.distance(rest, val))
                        .copied()
                        .take(5)
                        .collect()
                })
        }
    };
}

macro_rules! array_map_plugin_arbitrary {
    ($def:ident, $options:expr, $target:expr) => {
        pub fn $def<'a>(value: &SubjectValue, _theme: &'a TailwindTheme) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => $options
                    .iter()
                    .find(|(x, _)| x == v)
                    .map(|(_, y)| to_lit(&[($target, y)]))
                    .ok_or_else(|| {
                        let sort = eddie::Levenshtein::new();
                        $options
                            .iter()
                            .map(|(x, _)| x)
                            .sorted_by_key(|val| sort.distance(v, val))
                            .copied()
                            .take(5)
                            .collect()
                    }),
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
}

macro_rules! lookup_plugin_arbitrary {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(value: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => simple_lookup(&theme.$map, v, $target),
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(value: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
}

macro_rules! lookup_color_plugin_arbitrary {
    ($def:ident, $map:tt, $target:expr, $target2:expr) => {
        pub fn $def<'a>(
            value: &SubjectValue,
            theme: &'a TailwindTheme,
            alpha: Option<&Value>,
        ) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => {
                    simple_lookup_color(&theme.$map, v, $target, alpha, Some($target2))
                }
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(
            value: &SubjectValue,
            theme: &'a TailwindTheme,
            alpha: Option<&Value>,
        ) -> PluginResult<'a> {
            match value {
                SubjectValue::Value(Value(v)) => {
                    simple_lookup_color(&theme.$map, v, $target, alpha, None)
                }
                SubjectValue::Css(Css(v)) => Ok(to_lit(&[($target, v)])),
            }
        }
    };
}

macro_rules! lookup_plugin_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup(&theme.$map, rest.map(|v| v.0).unwrap_or("DEFAULT"), $target)
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(rest: Option<&Value>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            simple_lookup_map(
                &theme.$map,
                rest.map(|v| v.0).unwrap_or("DEFAULT"),
                $target,
                $closure,
            )
        }
    };
}

macro_rules! lookup_plugin_arbitrary_opt {
    ($def:ident, $map:tt, $target:expr) => {
        pub fn $def<'a>(
            value: Option<&SubjectValue>,
            theme: &'a TailwindTheme,
        ) -> PluginResult<'a> {
            match value {
                Some(SubjectValue::Value(Value(v))) => simple_lookup(&theme.$map, v, $target),
                Some(SubjectValue::Css(Css(v))) => Ok(to_lit(&[($target, v)])),
                // if there is no value, attempt to look up the default
                None => simple_lookup(&theme.$map, "DEFAULT", $target),
            }
        }
    };
    ($def:ident, $map:tt, $target:expr, $closure:expr) => {
        pub fn $def<'a>(
            value: Option<&SubjectValue>,
            theme: &'a TailwindTheme,
        ) -> PluginResult<'a> {
            match value {
                Some(SubjectValue::Value(Value(v))) => {
                    simple_lookup_map(&theme.$map, v, $target, $closure)
                }
                Some(SubjectValue::Css(Css(v))) => Ok(to_lit(&[($target, &$closure(v))])),
                // if there is no value, attempt to look up the default
                None => simple_lookup_map(&theme.$map, "DEFAULT", $target, $closure),
            }
        }
    };
}

macro_rules! merge_plugins {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: &Value, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

macro_rules! merge_plugins_arbitrary {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: &SubjectValue, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

macro_rules! merge_plugins_arbitrary_opt {
    ($def:ident, $closure_a:expr, $closure_b:expr) => {
        pub fn $def<'a>(rest: Option<&SubjectValue>, theme: &'a TailwindTheme) -> PluginResult<'a> {
            match ($closure_a(rest, theme), $closure_b(rest, theme)) {
                (Err(e1), Err(e2)) => Err(e1.into_iter().chain(e2).collect()),
                (Err(_), Ok(a)) => Ok(a),
                (Ok(b), Err(_)) => Ok(b),
                (Ok(a), Ok(b)) => Ok(merge_literals(a, b)),
            }
        }
    };
}

pub(crate) use array_map_plugin;
pub(crate) use array_map_plugin_arbitrary;
pub(crate) use array_plugin;
pub(crate) use lookup_plugin;
pub(crate) use lookup_plugin_arbitrary;
pub(crate) use lookup_plugin_arbitrary_opt;
pub(crate) use lookup_plugin_opt;
pub(crate) use merge_plugins;
pub(crate) use merge_plugins_arbitrary;
pub(crate) use merge_plugins_arbitrary_opt;
