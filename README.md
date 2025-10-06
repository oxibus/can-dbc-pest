# can-dbc-pest

[![GitHub repo](https://img.shields.io/badge/github-oxibus/can--dbc--pest-8da0cb?logo=github)](https://github.com/oxibus/can-dbc-pest)
[![crates.io version](https://img.shields.io/crates/v/can-dbc-pest)](https://crates.io/crates/can-dbc-pest)
[![crate usage](https://img.shields.io/crates/d/can-dbc-pest)](https://crates.io/crates/can-dbc-pest)
[![docs.rs status](https://img.shields.io/docsrs/can-dbc-pest)](https://docs.rs/can-dbc-pest)
[![crates.io license](https://img.shields.io/crates/l/can-dbc-pest)](https://github.com/oxibus/can-dbc-pest)
[![CI build status](https://github.com/oxibus/can-dbc-pest/actions/workflows/ci.yml/badge.svg)](https://github.com/oxibus/can-dbc-pest/actions)
[![Codecov](https://img.shields.io/codecov/c/github/oxibus/can-dbc-pest)](https://app.codecov.io/gh/oxibus/can-dbc-pest)

A CAN-dbc format parser written with Rust's [Pest](https://pest.rs/) grammar library. CAN databases are used to exchange details about a CAN network, e.g. what messages are being send over the CAN bus and what data do they contain.

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
