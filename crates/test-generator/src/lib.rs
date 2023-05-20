#![feature(path_file_prefix)]
use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};
use std::io::Write;
use swc_common::sync::Lrc;
use swc_common::Span;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_ast::{Lit, Module, ModuleItem};
use swc_ecma_parser::EsConfig;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Visit, VisitWith};

pub fn run(input: &str, output: &str) {
    let here = std::env::current_dir().unwrap();
    let path = here.join(input).canonicalize().unwrap();
    let out = here.join(output);
    std::fs::create_dir_all(&out).unwrap();
    let out = out.canonicalize().unwrap();

    println!("{:?} -> {:?}", path, out);

    let package_json = path.join("package.json");
    let package_json = std::fs::read_to_string(package_json).unwrap();
    if !package_json.contains("twin.macro") {
        panic!("package.json does not contain twin.macro");
    }

    let test_folder = path.join("tests/__fixtures__");
    let snapshots = path.join("tests/__snapshots__/plugin.test.js.snap");

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(&snapshots).expect("failed to load test.js");
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parse module");

    let contents = get_contents(&module).collect::<HashMap<_, _>>();

    let dirs = walkdir::WalkDir::new(test_folder);

    // ignore some tests
    let blacklist = [
        "pluginDaisyUi",
        "!imports",
        "!namelessImport",
        "!properties",
        "!variantGrouping",
        "addBase",
        "colorFunctions",
        "globalStyles",
        "pluginForms",
        "stitchesGlobals",
        "stitchesImports",
        "stitchesProps",
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    let mut modules = vec![];

    for dir in dirs {
        let entry = dir.unwrap();

        // get all tsx files
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|os| os.to_str()) != Some("tsx") {
            continue;
        }

        // get the file name
        let name = entry.path().file_prefix().unwrap().to_str().unwrap();

        if blacklist.contains(name) {
            continue;
        }

        let formatted_name = format!("twin.macro {}.tsx: {}.tsx 1", name, name);
        let content = contents.get(&formatted_name.as_str()).unwrap_or_else(|| {
            panic!(
                "failed to find content for {} in {:?}",
                formatted_name, contents
            )
        });

        let (input, output) = content.split_once("      ↓ ↓ ↓ ↓ ↓ ↓").unwrap();

        let input_mod = parse_module(&format!("{}In.tsx", name), input);
        let output_mod = parse_module(&format!("{}Out.tsx", name), output);

        let input = double_quote(input, &input_mod);
        let output = double_quote(output, &output_mod);

        if input_mod.body.len() != output_mod.body.len() {
            panic!(
                "{}: snapshots with different number of statements not supported: {} vs {}",
                name,
                input_mod.body.len(),
                output_mod.body.len()
            );
        }

        let pairs = input_mod
            .body
            .iter()
            .zip(output_mod.body.iter())
            .map(|(l, r)| (read_span(&input, l), read_span(&output, r)));

        let module_name = match entry
            .path()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
        {
            Some(parent) => format!("{}_{}", parent, name),
            None => name.to_string(),
        }
        .to_case(Case::Snake);
        let module_name = module_name.replace('!', "");
        let out = out.join(format!("{}.rs", module_name));

        // Open a file to write
        let mut out_file = std::fs::File::create(out).expect("Cannot create file");

        writeln!(
            out_file,
            r#"use crate::test::snapshot_inner;
use test_case::test_case;"#
        )
        .expect("Cannot write to file");

        // For each pair of input and output
        for (i, (input, output)) in pairs
            .filter(|(input, _)| !input.starts_with("import"))
            .enumerate()
        {
            // Write the formatted test case to the file
            writeln!(
                out_file,
                "#[test_case(r#####\"{}\"#####, r#####\"{}\"##### ; \"{}\")]",
                input, output, i
            )
            .expect("Cannot write to file");
        }

        writeln!(
            out_file,
            r#"fn test(input: &str, output: &str) {{
    snapshot_inner(input, output)
}}"#
        )
        .expect("Cannot write to file");

        modules.push(module_name);
    }

    let out = out.join("mod.rs");
    let mut out_file = std::fs::File::create(out).expect("Cannot create file");
    for module in modules {
        writeln!(out_file, "mod {};", module).expect("Cannot write to file");
    }
}

struct RespanVisitMut<'a>(&'a str, Option<&'a str>);

impl<'a> Visit for RespanVisitMut<'a> {
    fn visit_span(&mut self, span: &Span) {
        if self.1.is_some() {
            return;
        }
        self.1 = Some(&self.0[(span.lo.0 as usize - 1)..span.hi.0 as usize - 1]);
    }

    fn visit_module_item(&mut self, n: &ModuleItem) {
        n.visit_children_with(self);
    }
}

fn read_span<'a>(input: &'a str, span: &ModuleItem) -> &'a str {
    let mut v = RespanVisitMut(input, None);
    v.visit_module_item(span);
    v.1.unwrap()
}

struct SingleQuoteIndices(Vec<usize>);

impl Visit for SingleQuoteIndices {
    fn visit_lit(&mut self, s: &Lit) {
        if let Lit::Str(s) = s {
            if let Some('\'') = s.raw.as_ref().and_then(|r| r.chars().next()) {
                self.0.push(s.span.lo.0 as usize - 1);
                self.0.push(s.span.hi.0 as usize - 2);
            }
        }
    }
}

fn double_quote(input: &str, module: &Module) -> String {
    let mut indices = SingleQuoteIndices(vec![]);
    indices.visit_module(&module);

    let mut bytes = input.to_owned().into_bytes();
    for x in indices.0 {
        bytes[x] = 0x22;
    }

    String::from_utf8(bytes).unwrap()
}

fn get_contents(stmt: &Module) -> impl Iterator<Item = (&str, String)> {
    stmt.body.iter().filter_map(|mi| {
        let assign = mi.as_stmt()?.as_expr()?.expr.as_assign()?;
        let member = assign.left.as_pat()?.as_expr()?.as_member()?;

        let name = &member
            .prop
            .as_computed()?
            .expr
            .as_tpl()?
            .quasis
            .first()?
            .raw;

        let contents = &assign.right.as_tpl()?.quasis.first()?.raw;
        let contents = unescape_backticks(contents);

        Some((name.as_ref(), contents))
    })
}

fn unescape_backticks(s: &str) -> String {
    use aho_corasick::{AhoCorasick, MatchKind};

    let patterns = &["\\`", "\\$", "\\\\\""];
    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns)
        .unwrap();

    ac.replace_all(s, &["`", "$", "\\\""])
}

fn parse_module(file_name: &str, file: &str) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Custom(file_name.into()), file.into());
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parse module")
}
