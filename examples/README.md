# Examples

If the crate is compiled as (from the crate root dir):

```sh
rustc --edition=2024 --crate-type bin -L dependency=target/debug/deps/ --extern rand=target/debug/deps/librand-90518233a81aa76c.rlib src/main.rs
```

then run VerifOpt on it as (from the VerifOpt root dir):

```sh
cargo run --release -- -C opt-level=3 --edition=2024 --crate-type bin -L dependency=../examples/two_variants_rand/target/debug/deps/ --extern rand=../examples/two_variants_rand/target/debug/deps/librand-90518233a81aa76c.rlib ../examples/two_variants_rand/src/main.rs
```
