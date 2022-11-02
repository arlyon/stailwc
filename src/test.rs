use std::{fs::read_to_string, path::Path};

use cmd_lib::run_fun;
use swc_core::{
    common::errors::{ColorConfig, Handler},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::testing::{test, test_transform},
        visit::as_folder,
    },
    plugin::errors::HANDLER,
};

use crate::{
    config::{AppConfig, Screens},
    TransformVisitor,
};

fn test_visitor() -> TransformVisitor<'static> {
    let mut t = TransformVisitor::default();

    t.config.theme.height.insert("4", "1rem");
    t.config.theme.spacing.insert("4", "1rem");
    t.config.theme.colors.insert("black", "black");
    t.config.theme.margin.insert("4", "1rem");
    t.config.theme.screens.insert("lg", Screens::Min("1024px"));

    t
}

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    basic,
    // Input codes
    r#"<Test tw="h-4" />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{height: "1rem"}} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    string,
    // Input codes
    r#"<Test tw={"h-4"} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{height: "1rem"}} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    template,
    r#"const x = tw`h-4`"#,
    r#"const x = {height: "1rem"}"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    template_jsx,
    r#"<Test css={tw`h-4`} />"#,
    r#"<Test css={{height: "1rem"}} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    existing_css_object,
    // Input codes
    r#"<Test tw="h-4" css={{width: "1rem"}} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{width: "1rem"}, {height: "1rem"}]} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    existing_css_array,
    // Input codes
    r#"<Test tw="h-4" css={[]} />"#,
    // Output codes after transformed with plugin
    r#"<Test css={[{height: "1rem"}]} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    multiple_mod,
    // Input codes
    r#"<Test tw="hover:h-4 hover:text-black" />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{":hover": {height: "1rem", color: "black"}}} />"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(test_visitor()),
    minus_values,
    // Input codes
    r#"<Test tw="-m-4" />"#,
    // Output codes after transformed with plugin
    r#"<Test css={{margin: "-1rem"}} />"#
);

include!(concat!(env!("OUT_DIR"), "/test_cases.rs"));

fn snapshots_inner(path: &str) {
    let input_path = Path::new(path);
    let snapshot_path = Path::new("snapshots/output").join(input_path.file_name().unwrap());
    let snapshot = read_to_string(snapshot_path).unwrap();

    if let Err(e) = HANDLER.inner.set(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        None,
    )) {
        // set on a previous run
    }

    let tailwind_path = input_path.with_file_name("tailwind.config.js");
    let tailwind_path = if tailwind_path.exists() {
        format!("'./{}'", tailwind_path.to_str().unwrap())
    } else {
        "undefined".to_string()
    };

    let (input, expected) = snapshot.split_once("\n      ↓ ↓ ↓ ↓ ↓ ↓\n").unwrap();

    let app_config =
        run_fun!(node -e "console.log(JSON.stringify(require('./install.js')({silent: true, tailwindPath: ${tailwind_path}})[1]))")
            .unwrap();

    let deser = &mut serde_json::Deserializer::from_str(&app_config);
    let app_config: Result<AppConfig, _> = serde_path_to_error::deserialize(deser);

    test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| {
            as_folder(TransformVisitor {
                config: app_config.unwrap().config,
                tw_attr: None,
                tw_tpl: None,
            })
        },
        input,
        expected,
        false,
    )
}
