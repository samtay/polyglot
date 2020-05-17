# rust
Crack the Coding Interview solutions in Rust

**Disclaimer**: I am using this as an opportunity to learn Rust, so if anyone
happens upon this repo, there may be non-idiomatic and/or flat-out-bad code!

### set up
To solve more problems, first add a chapter directory (e.g.
[chapter1](./src/chapter1)) and associated [mod.rs](./src/chapter1/mod.rs).
Then just add individual exercises as modules within the chapter directory,
exporting them as public modules from the `mod.rs` file.

Write tests first, in the same file as the exercise. Make sure
[cargo-watch](https://github.com/passcod/cargo-watch) is installed. Then
```
cargo watch -x test
```
to iterate on a solution.
