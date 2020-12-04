use crate::{build_plan::BuildPlan, error::Error};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process,
};

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
        let env = Self::read_env(&platform_path)?;

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

    /// Read User Environment into a HashMap. Returns an empty HashMap if the path does not exist.
    fn read_env(path: impl AsRef<Path>) -> Result<HashMap<String, String>, Error> {
        let path = path.as_ref();
        let mut env = HashMap::new();

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                let key_os = match path.file_stem() {
                    Some(key_os) => key_os,
                    None => continue,
                };
                if let Some(key) = key_os.to_str() {
                    let value = fs::read_to_string(&path)?;
                    env.insert(key.to_string(), value);
                }
            }
        }

        Ok(env)
    }
}
