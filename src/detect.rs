use crate::build_plan::BuildPlan;
use std::{env::Args, fs, path::PathBuf, process};

const PASS_EXIT_CODE: i32 = 0;
const FAIL_EXIT_CODE: i32 = 100;

/// Buildpack API Detect
pub struct Detect {
    /// path to platform directory
    pub platform_path: PathBuf,
    /// Build Plan (TOML)
    build_plan_path: PathBuf,
}

impl Detect {
    pub fn new(platform: impl Into<PathBuf>, build_plan: impl Into<PathBuf>) -> Self {
        Detect {
            platform_path: platform.into(),
            build_plan_path: build_plan.into(),
        }
    }

    pub fn from_args(mut args: Args) -> Option<Self> {
        match (args.next(), args.next()) {
            (Some(platform), Some(build_plan)) => Some(Self::new(platform, build_plan)),
            _ => None,
        }
    }

    pub fn write_buildplan(&self, build_plan: &BuildPlan) -> anyhow::Result<()> {
        fs::write(&self.build_plan_path, toml::to_string(build_plan)?)?;

        Ok(())
    }

    pub fn pass(&self) {
        process::exit(PASS_EXIT_CODE);
    }

    pub fn fail(&self) {
        process::exit(FAIL_EXIT_CODE);
    }

    /// error code is 1-99 or 101+
    pub fn error(&self, code: i32) {
        process::exit(code);
    }
}
