# `cargo xtask` Example

![License](https://img.shields.io/github/license/nickgerace/cargo-xtask-example?style=for-the-badge&color=blue)
![Language](https://img.shields.io/github/languages/top/nickgerace/cargo-xtask-example?&logo=rust&color=orange&style=for-the-badge)

This is an example repository for `cargo xtask`.
For more information on [@matklad](https://github.com/matklad)'s `cargo xtask` pattern, visit the [original repository](https://github.com/matklad/cargo-xtask).

## Quickstart

In the root of the repository, run the following command:

```shell
cargo xtask
```

A list of "tasks" should print alongside their descriptions.
This command works via an alias in the repository's [cargo configuration file](.cargo/config).
The alias allows you to run tasks from any directory within the repository, like other command runners.

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
    │       └── main.rs
    │
    └── your-crate
        ├── Cargo.toml
        └── src
    │       └── main.rs
```

## Why `cargo xtask`?

The `cargo xtask` pattern provides repository automation without needing to install another dependency.
If a contributor can compile Rust code, they can use `cargo xtask`.