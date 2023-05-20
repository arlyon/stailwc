#![feature(path_file_prefix)]

use clap::Parser as ClapParser;

// clap derive with one path
#[derive(ClapParser)]
#[clap(name = "test-generator", version = "0.1.0", author = "")]
struct Opts {
    /// Path to the twin.macro generated tests
    input: String,

    #[clap(default_value = "out")]
    output: String,
}

fn main() {
    let opts = Opts::parse();
    test_generator::run(&opts.input, &opts.output);
}
