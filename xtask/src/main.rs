//! This crate is an example of the [`cargo-xtask`](https://github.com/matklad/cargo-xtask) pattern.

use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const RELEASE_SIZE_CRATE_NAME: &str = "your-crate";
const PRINT_PREFIX: &str = "[xtask]";

// TODO(nick): auto-generate these without introducing another crate.
const HELP_TEXT: &str = "\
Available tasks:
  bloat
    scan for potential bloat
  build
    build all targets
  build-release
    build all targets, scan and check binary size  
  ci
    run the ci suite
  prepare
    run update, format code, and fix lints
  scan
    scan for vulnerabilities, outdated, unused dependencies";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let maybe_argument = env::args()
        .nth(1)
        .as_ref()
        .map(|argument| argument.trim().to_lowercase());

    let runner = TaskRunner::new()?;

    match maybe_argument.as_deref() {
        Some("bloat") => runner.task_bloat(),
        Some("build") => runner.task_build(),
        Some("build-release") => runner.task_build_release(),
        Some("ci") => runner.task_ci(),
        Some("prepare") => runner.task_prepare(),
        Some("scan") => runner.task_scan(),
        Some(invalid_task) => {
            eprintln!("{HELP_TEXT}");
            Err(format!("invalid task: {invalid_task}").into())
        }
        None => {
            println!("{HELP_TEXT}");
            Ok(())
        }
    }
}

struct TaskRunner {
    root: PathBuf,
}

impl TaskRunner {
    pub fn new() -> Result<Self> {
        let root = match Path::new(CARGO_MANIFEST_DIR).ancestors().nth(1) {
            Some(found_root) => found_root.to_path_buf(),
            None => return Err("could not determine repo root".into()),
        };
        Ok(Self { root })
    }

    fn cargo(&self, args: &'static str) -> Result<()> {
        self.stdout(format!("running: cargo {args}"));

        let mut cmd = Command::new("cargo");
        match cmd
            .current_dir(&self.root)
            .args(args.trim().split(" "))
            .status()?
            .success()
        {
            true => Ok(()),
            false => Err("cargo command failed".into()),
        }
    }

    fn release_size(&self) -> Result<u64> {
        Ok(File::open(
            self.root
                .join("target")
                .join("release")
                .join(RELEASE_SIZE_CRATE_NAME),
        )?
        .metadata()?
        .len())
    }

    fn stdout(&self, contents: impl AsRef<str>) {
        let contents = contents.as_ref();
        println!("{PRINT_PREFIX} {contents}");
    }

    #[allow(dead_code)]
    fn stderr(&self, contents: impl AsRef<str>) {
        let contents = contents.as_ref();
        eprintln!("{PRINT_PREFIX} {contents}");
    }

    pub fn task_bloat(&self) -> Result<()> {
        self.cargo("bloat --release")?;
        self.cargo("bloat --release --crates")?;
        Ok(())
    }

    pub fn task_build(&self) -> Result<()> {
        self.cargo("build --all-targets")?;
        Ok(())
    }

    pub fn task_build_release(&self) -> Result<()> {
        self.task_scan()?;
        self.cargo("build --release")?;
        self.stdout(format!("binary size: {}", self.release_size()?));
        Ok(())
    }

    pub fn task_ci(&self) -> Result<()> {
        self.cargo("fmt --all -- --check")?;
        self.cargo("check --all-targets --all-features --workspace")?;
        self.cargo("clippy --all-targets --all-features --no-deps --workspace -- -D warnings")?;
        self.cargo("test")?;
        Ok(())
    }

    pub fn task_prepare(&self) -> Result<()> {
        self.cargo("update")?;
        self.cargo("fmt")?;
        self.cargo("fix --edition-idioms --allow-dirty --allow-staged --workspace")?;
        self.cargo("clippy --fix --all-features --all-targets --no-deps --workspace")?;
        Ok(())
    }

    pub fn task_scan(&self) -> Result<()> {
        self.cargo("outdated")?;
        self.cargo("+nightly udeps")?;
        self.cargo("audit")?;
        Ok(())
    }
}
