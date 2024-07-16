<!-- markdownlint-disable MD041 -->
[![Build and test](https://github.com/EnergySystemsModellingLab/highs-example-rust/actions/workflows/cargo-build-and-test.yml/badge.svg)](https://github.com/EnergySystemsModellingLab/highs-example-rust/actions/workflows/cargo-build-and-test.yml)
[![GitHub](https://img.shields.io/github/license/EnergySystemsModellingLab/highs-example-rust)](https://raw.githubusercontent.com/EnergySystemsModellingLab/highs-example-rust/main/LICENSE)

# HiGHS example in Rust

This repository provides a minimal example of using the [HiGHS solver] from Rust, for which we use
the [`highs`] crate. The program settings are read from a TOML file, which references CSV files
containing variable definitions and constraints ([see example]).

[HiGHS solver]: https://highs.dev
[`highs`]: https://crates.io/crates/highs
[see example]: ./example

## Getting started

### Installing the Rust toolchain

We recommend that developers use `rustup` to install the Rust toolchain. Follow the instructions on
[the `rustup` website](https://rustup.rs/).

Once you have done so, select the `stable` toolchain (used by this project) as your default with:

```sh
rustup default stable
```

### Working with the project

To build the project, run:

```sh
cargo build
```

To run MUSE with the example input files, you can run:

```sh
cargo run example/settings.toml
```

Tests can be run with:

```sh
cargo test
```

More information is available in [the official `cargo` book](https://doc.rust-lang.org/cargo/).

## Copyright

Copyright © 2024 Imperial College London
