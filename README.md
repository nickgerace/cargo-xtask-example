# cargo-xtask-example

![License](https://img.shields.io/github/license/nickgerace/cargo-xtask-example?style=flat-square&color=blue)
![Language](https://img.shields.io/github/languages/top/nickgerace/cargo-xtask-example?&logo=rust&color=orange&style=flat-square)

This is an example repository for `cargo xtask`.
For more information on [@matklad](https://github.com/matklad)'s `cargo xtask` pattern, visit the [original repository](https://github.com/matklad/cargo-xtask).

## Quickstart

In the root of the repository, run the following command:

```shell
cargo xtask
```

A list of "tasks" should print alongside their descriptions.
This command works via an alias in the repository's [cargo configuration file](.cargo/config).

## Repository Structure

This repository contains two crates: `xtask` and `your-crate`.
The former contains the "tasks" and ability to run them.
The latter represents the crate(s) that `xtask` would help orchestrate "tasks" for.

```
.
├── .cargo
│   └── config
│
├── Cargo.lock
├── Cargo.toml
│
├── README.md
│
└── crates
    │
    ├── xtask
    │   ├── Cargo.toml
    │   └── src
    │
    └── your-crate
        ├── Cargo.toml
        └── src
```

## Why `cargo xtask`?

Using external build systems and scripting languages can be useful, but using these technologies can result in inaccessible contributing experiences and potentially locking out valid development environments.

Since `cargo` _is_ the tried and true build system for Rust (tested on multiple tiered targets), we can get the best of both worlds by using a small wrapper around it.
Thus, `cargo xtask` exists to fill the gap; allowing for repository automation without needing to install another dependency.
