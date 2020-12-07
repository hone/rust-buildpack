use crate::{buildpack_plan::BuildpackPlan, error::Error, user_env};
use std::{collections::HashMap, fs, path::PathBuf};

/// Build
pub struct Build {
    pub platform_path: PathBuf,
    pub buildpack_plan: BuildpackPlan,
    pub user_env: HashMap<String, String>,
    buildpack_plan_path: PathBuf,
    layers_path: PathBuf,
}

impl Build {
    pub fn new(
        layers_path: impl Into<PathBuf>,
        platform_path: impl Into<PathBuf>,
        buildpack_plan_path: impl Into<PathBuf>,
    ) -> Result<Self, Error> {
        let platform_path = platform_path.into();
        let env = user_env::read_env(&platform_path)?;
        let buildpack_plan_path = buildpack_plan_path.into();
        let buildpack_plan = toml::from_str(&fs::read_to_string(&buildpack_plan_path)?)?;

        Ok(Build {
            platform_path,
            buildpack_plan,
            buildpack_plan_path,
            layers_path: layers_path.into(),
            user_env: env,
        })
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Option<Self>, Error> {
        match (args.next(), args.next(), args.next()) {
            (Some(layers), Some(platform), Some(buildpack_plan)) => {
                Ok(Some(Self::new(layers, platform, buildpack_plan)?))
            }
            _ => Ok(None),
        }
    }
}
