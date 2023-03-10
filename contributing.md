# Contributing to async-stripe

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Test it: `cargo test --workspace`
4. Lint it: `cargo +nightly clippy --all --all-targets -- -D warnings`
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin my-new-feature`
7. Submit a pull request :D

We use `rustfmt` to keep our codebase consistently formatted. Please ensure that
you have correctly formatted your code (most editors will do this automatically
when saving) or it may not pass the CI tests.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as in the README, without any
additional terms or conditions.

## Building

To get started you will need a rust toolchain which can cross-compile to the 
`wasm32-wasi` target. In addition, there is currently an issue where newer
rust toolchains cause an out-of-bounds memory access. To remedy this, we have
pinned the toolchain to a specific known-working nightly. The solution can
be tracked here: https://github.com/swc-project/swc/issues/6807#issuecomment-1463706888

```sh
❯ rustup override set nightly-2022-11-23
❯ rustup target add wasm32-wasi
```

From there you can run a cargo build

```sh
❯ cargo build --release --target wasm32-wasi
❯ file target/wasm32-wasi/release/stailwc.wasm
target/wasm32-wasi/release/stailwc.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

## Coding standards

These are requirements we have that we have not yet lifted to the level of
automatic enforcement.

### Import grouping

In each file the imports should be grouped into at most 4 groups in the
following order:

1. stdlib
2. non-repository local crates
3. repository local other crates
4. this crate

Separate each group with a blank line, and rustfmt will sort into a canonical
order. Any file that is not grouped like this can be rearranged whenever the
file is touched - we're not precious about having it done in a separate commit,
though that is helpful.

### Clippy lints

We ask that contributors keep the clippy status clean. Minimally, run `cargo clippy`
before submitting code. Clippy is also run in GitHub Actions.

### rustfmt

It is expected that code is uniformly formatted. Before submitting code, make sure
to run `cargo fmt` to make sure it conforms to the standard.

## Communication

It is encouraged to open an issue before you create a PR as a place for pre-implementation
discussion. If you're unsure about your contribution or simply want to ask a question about anything just open an issue and we'll chat.

## Architecture

