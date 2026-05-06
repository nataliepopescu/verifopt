# Monomorphization using Project Stable MIR

## Run Example

```sh
cargo run --release -- ../analysis/examples/generic_res.rs --crate-name generic_res --edition 2021 -C panic=abort -C opt-level=3
```

## Results

If you pipe the stdout from the above command and inspect the MIR for the `foo`
function, you can see that it is monomorphized to the `Rect` type

