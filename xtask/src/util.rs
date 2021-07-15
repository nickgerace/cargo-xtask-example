use anyhow::{anyhow, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const NAME: &str = "example";

pub fn root() -> Option<PathBuf> {
    // The "env!" macro loads the environment variable at compile time. This is fine because we are
    // using this function within an xtask.
    Some(
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)?
            .to_path_buf(),
    )
}

pub fn binary() -> Option<PathBuf> {
    Some(root()?.clone().join("target").join("release").join(NAME))
}

pub fn human_readable_size(b: f64) -> String {
    fn convert_to_human_bytes(b: f64, tier: u8) -> (f64, u8) {
        match b > 1024.0 {
            true => convert_to_human_bytes(b / 1024.0, tier + 1),
            false => (b, tier),
        }
    }
    let (size, tier) = convert_to_human_bytes(b, 0);
    format!(
        "{:.2} {}",
        size,
        match tier {
            0 => "B",
            1 => "KB",
            2 => "MB",
            3 => "GB",
            _ => "you serious right now?",
        }
    )
}

pub fn exec(command: &str, args: Option<Vec<&str>>) -> Result<()> {
    match command.is_empty() {
        true => Err(anyhow!("command must not be empty")),
        false => {
            let mut cmd = Command::new(command);
            cmd.current_dir(match root() {
                Some(s) => s,
                None => return Err(anyhow!("could not determine repository root")),
            });
            match args {
                Some(s) => {
                    println!("{} {}", command, s.join(" "));
                    cmd.args(s);
                }
                None => println!("{}", command),
            }
            match cmd.status()?.success() {
                true => Ok(()),
                false => Err(anyhow!("command failed")),
            }
        }
    }
}
