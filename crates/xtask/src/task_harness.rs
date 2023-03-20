use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{TaskError, TaskResult};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CRATE: &str = "your-crate";
const PRINT_PREFIX: &str = "[xtask]";

pub struct TaskHarness {
    root: PathBuf,
}

impl TaskHarness {
    pub fn new() -> TaskResult<Self> {
        let root = match Path::new(CARGO_MANIFEST_DIR).ancestors().nth(2) {
            Some(found_root) => found_root.to_path_buf(),
            None => return Err(TaskError::CouldNotDetermineRepositoryRoot),
        };
        Ok(Self { root })
    }

    fn cargo(&self, args: &'static str) -> TaskResult<()> {
        self.stdout(format!("running: cargo {args}"));

        let mut cmd = Command::new("cargo");
        match cmd
            .current_dir(&self.root)
            .args(args.trim().split(" "))
            .status()?
            .success()
        {
            true => Ok(()),
            false => Err(TaskError::CargoCommandFailed),
        }
    }

    pub fn task_bloat(&self) -> TaskResult<()> {
        self.cargo("bloat --release")?;
        self.cargo("bloat --release --crates")
    }

    pub fn task_build(&self) -> TaskResult<()> {
        self.task_prepare()?;
        self.cargo("build --all-targets")
    }

    pub fn task_build_release(&self) -> TaskResult<()> {
        self.task_prepare()?;
        self.task_scan()?;
        self.cargo("build --release")?;
        self.stdout(format!("binary size: {}", self.release_size()?));
        Ok(())
    }

    pub fn task_ci(&self) -> TaskResult<()> {
        self.cargo("fmt --all -- --check")?;
        self.cargo("check --all-targets --all-features")?;
        self.cargo("clippy --all-targets --all-features --no-deps -- -D warnings")?;
        self.cargo("test -- --nocapture")?;
        Ok(())
    }

    pub fn task_prepare(&self) -> TaskResult<()> {
        self.cargo("update")?;
        self.cargo("fmt")?;
        self.cargo("fix --edition-idioms --allow-dirty --allow-staged")?;
        self.cargo("clippy --all-features --all-targets --no-deps")
    }

    pub fn task_scan(&self) -> TaskResult<()> {
        self.cargo("+nightly udeps")?;
        self.cargo("audit")
    }

    fn release_size(&self) -> TaskResult<u64> {
        Ok(
            File::open(self.root.join("target").join("release").join(CRATE))?
                .metadata()?
                .len(),
        )
    }

    pub fn stdout(&self, contents: impl AsRef<str>) {
        let contents = contents.as_ref();
        println!("{PRINT_PREFIX} {contents}")
    }

    #[allow(dead_code)]
    pub fn stderr(&self, contents: impl AsRef<str>) {
        let contents = contents.as_ref();
        eprintln!("{PRINT_PREFIX} {contents}")
    }
}
