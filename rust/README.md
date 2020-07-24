# rust

### set up
To solve more CTCI problems, first add a chapter directory (e.g.
[chapter1](./src/ctci/chapter1)) and associated [mod.rs](./src/ctci/chapter1/mod.rs).
Then just add individual exercises as modules within the chapter directory,
exporting them as public modules from the `mod.rs` file.

Write tests first, in the same file as the exercise. Make sure
[cargo-watch](https://github.com/passcod/cargo-watch) is installed. Then
```
cargo watch -x test
```
to iterate on a solution.

#### nightly
To use nightly features:

1. Add them at the [crate level](./src/lib.rs)
2. Use the nightly toolchain, e.g. `rustup override set nightly`
3. Run with `nightly` feature flag, e.g. `cargo watch -x 'test --lib --features=nightly' -x 'clippy'`
