use crate::util;
use anyhow::{anyhow, Result};
use std::fs::File;

pub fn build() -> Result<()> {
    prepare()?;
    let binary = match util::binary() {
        Some(s) => s,
        None => return Err(anyhow!("could not determine binary location")),
    };

    util::exec("cargo", Some(vec!["build", "--release"]))?;
    println!(
        "{}",
        util::human_readable_size(File::open(&binary)?.metadata()?.len() as f64)
    );

    #[cfg(not(target_os = "windows"))]
    util::exec("strip", Some(vec![&binary.to_str().unwrap()]))?;
    #[cfg(not(target_os = "windows"))]
    println!(
        "{} (post strip)",
        util::human_readable_size(File::open(&binary)?.metadata()?.len() as f64)
    );

    Ok(())
}

pub fn default_run() -> Result<()> {
    prepare()?;
    util::exec("cargo", Some(vec!["run"]))?;
    Ok(())
}

pub fn dry_run() -> Result<()> {
    prepare()?;
    util::exec("cargo", Some(vec!["run", "--", "run", "--dry-run"]))?;
    Ok(())
}

pub fn full_prepare() -> Result<()> {
    util::exec("cargo", Some(vec!["update"]))?;
    prepare()?;
    util::exec("cargo", Some(vec!["+nightly", "udeps"]))?;
    Ok(())
}

pub fn prepare() -> Result<()> {
    util::exec("cargo", Some(vec!["+nightly", "fmt"]))?;
    util::exec("cargo", Some(vec!["clippy"]))?;
    Ok(())
}
