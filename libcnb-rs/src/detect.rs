use crate::{build_plan::BuildPlan, error::Error, user_env};
use std::{collections::HashMap, fs, path::PathBuf, process};

const PASS_EXIT_CODE: i32 = 0;
const FAIL_EXIT_CODE: i32 = 100;

/// Buildpack API Detect
pub struct Detect {
    /// path to platform directory
    pub platform_path: PathBuf,
    /// Build Plan (TOML)
    build_plan_path: PathBuf,
    /// User Environment
    pub user_env: HashMap<String, String>,
}

impl Detect {
    pub fn new(
        platform: impl Into<PathBuf>,
        build_plan: impl Into<PathBuf>,
    ) -> Result<Self, Error> {
        let platform_path = platform.into();
        let env = user_env::read_env(&platform_path)?;

        Ok(Detect {
            platform_path,
            build_plan_path: build_plan.into(),
            user_env: env,
        })
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Option<Self>, Error> {
        match (args.next(), args.next()) {
            (Some(platform), Some(build_plan)) => Ok(Some(Self::new(platform, build_plan)?)),
            _ => Ok(None),
        }
    }

    pub fn write_buildplan(&self, build_plan: &BuildPlan) -> Result<(), Error> {
        fs::write(&self.build_plan_path, toml::to_string(build_plan)?)?;

        Ok(())
    }

    pub fn pass(&self, build_plan: Option<&BuildPlan>) -> Result<(), Error> {
        if let Some(build_plan) = build_plan {
            fs::write(&self.build_plan_path, toml::to_string(build_plan)?)?;
        }

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
