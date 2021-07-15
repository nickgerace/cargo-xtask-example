use anyhow::{anyhow, Result};
use std::env;

mod task;
mod util;

const HELP_TEXT: &str = "\
TASK            DESCRIPTION
build           builds a release binary
default-run     runs with defaults
dry-run         performs a dry run
full-prepare    updates deps, formats code, runs clippy, and checks for udeps
prepare         formats code and runs clippy";

fn main() -> Result<()> {
    match env::args().nth(1).as_ref().map(|arg| arg.as_str()) {
        Some("build") => task::build(),
        Some("default-run") => task::default_run(),
        Some("dry-run") => task::dry_run(),
        Some("full-prepare") => task::full_prepare(),
        Some("prepare") => task::prepare(),
        Some(_) => {
            eprintln!("{}", HELP_TEXT);
            Err(anyhow!("invalid xtask"))
        }
        None => {
            println!("{}", HELP_TEXT);
            Ok(())
        }
    }
}
