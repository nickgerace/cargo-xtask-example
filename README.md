# cargo-xtask-example

![License](https://img.shields.io/github/license/nickgerace/cargo-xtask-example?style=flat-square&color=blue)
![Language](https://img.shields.io/github/languages/top/nickgerace/cargo-xtask-example?&logo=rust&color=orange&style=flat-square)

This is an example repository for `cargo xtask`.
For more information on [@matklad](https://github.com/matklad)'s `cargo xtask` pattern, visit the [original repository](https://github.com/matklad/cargo-xtask).

## Why `cargo xtask`?

Using external build systems and scripting languages can be useful, but using these technologies can result in inaccessible contributing experiences and potentially locking out valid development environments.

Since `cargo` _is_ the tried and true build system for Rust (tested on multiple tiered targets), we can get the best of both worlds by using a small wrapper around it.
Thus, `cargo xtask` exists to fill the gap; allowing for repository automation without needing to install another dependency.