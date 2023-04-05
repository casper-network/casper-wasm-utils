# casper-wasm-utils

A collection of WASM utilities used in contract development, forked from the Parity's `pwasm-utils`.
The original code was written by the Parity team, and this library is a fork allowing Casper to maintain the code.

This repository contains the package `casper-wasm-utils` which consists of a library crate
and a collection of cli binaries that make use of this library.

## Installation of cli tools

```
cargo install pwasm-utils --features cli
```

This will install the following binaries:

-   wasm-gas
-   wasm-stack-height

# License

`wasm-utils` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `wasm-utils` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
