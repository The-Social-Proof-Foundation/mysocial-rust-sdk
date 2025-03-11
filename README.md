# MySocial Rust SDK

The MySocial Rust SDK for integrating with the [MySocial blockchain](https://docs.mso.io/).

> [!NOTE]
> This is project is under development and many features may still be under
> development or missing.

## Overview

This repository contains a collection of libraries for integrating with the MySocial blockchain.

A few of the project's high-level goals are as follows:

* **Be modular** - user's should only need to pay the cost (in terms of dependencies/compilation time) for the features that they use.
* **Be light** - strive to have a minimal dependency footprint.
* **Support developers** - provide all needed types, abstractions and APIs to enable developers to build robust applications on MySocial.
* **Support wasm** - where possible, libraries should be usable in wasm environments.

## Crates

In an effort to be modular, functionality is split between a number of crates.

* [`mso-sdk-types`](crates/mso-sdk-types)
    [![mso-sdk-types on crates.io](https://img.shields.io/crates/v/mso-sdk-types)](https://crates.io/crates/mso-sdk-types)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mso-sdk-types)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mso-rust-sdk/mso_sdk_types/)
* [`mso-crypto`](crates/mso-crypto)
    [![mso-crypto on crates.io](https://img.shields.io/crates/v/mso-crypto)](https://crates.io/crates/mso-crypto)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mso-crypto)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mso-rust-sdk/mso_crypto/)
* [`mso-graphql-client`](crates/mso-crypto)
    [![mso-graphql-client on crates.io](https://img.shields.io/crates/v/mso-graphql-client)](https://crates.io/crates/mso-graphql-client)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mso-graphql-client)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mso-rust-sdk/mso-graphql-client/)

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
