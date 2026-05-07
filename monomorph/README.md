# VerifOpt on Monomorphized MIR using Project Stable MIR

## Requirements

* Rust nightly and components, as specified in [rust-toolchain](rust-toolchain.toml).

## Build

1. Clone the repository

2. Build & install
    
    You can build VerifOpt in two different ways:

    ```sh
    cargo build
    ```
    
    This command generates two binaries, `cargo-verifopt` and `verifopt`, in the `target/debug` directory.

    You can also install VerifOpt into `cargo`:

    ```sh
    cargo --locked install --path .
    ```
    
    This enables you to perform flow-sensitive analysis on a Rust project using the command `cargo verifopt`, similar to other `cargo` commands such as `cargo fmt`.

## Usage

You can run VerifOpt for **a Rust project** using the binary `cargo-pta`:

```sh
cargo-verifopt verifopt
```

You can also use the command `cargo verifopt` instead of `cargo-verifopt verifopt` if VerifOpt has been installed into `cargo`.

Alternatively, you can run VerifOpt for **a single file** using the binary
`verifopt`:
    
```sh
verifopt <path-to-file>
```

## LOG

Set the `VERIFOPT_LOG` environment variable to enable logging:

```sh
export VERIFOPT_LOG=info
```

## Troubleshooting

If you encounter errors loading shared libraries, such as `librustc_driver.so`, try setting:

```sh
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```

