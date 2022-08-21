use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use glob::glob;

fn main() {
    let test_files = glob("tests/**/*.js")
        .expect("Failed to read glob pattern")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|s| !s.ends_with("tailwind.config.js") && !s.ends_with("stitches.config.js"));

    let outfile_path = Path::new(&env::var("OUT_DIR").unwrap()).join("test_cases.rs");
    let mut outfile = File::create(outfile_path).unwrap();

    for file in test_files {
        let path = file.to_str().unwrap();
        let name = path.replace(".js", "");

        write!(
            outfile,
            r#"#[test_case::test_case("{}" ; "snapshot {}")]
            "#,
            path, name
        )
        .unwrap();
    }

    write!(
        outfile,
        r#"#[ignore]
        fn snapshots(path: &str) {{
        snapshots_inner(path)
    }}"#,
    )
    .unwrap()
}
