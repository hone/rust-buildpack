use libcnb_rs::{
    build_plan::{BuildPlan, Provide, Require},
    detect::Detect,
};

const ARGS_EXIT_CODE: i32 = 101;
const BUILDPLAN_EXIT_CODE: i32 = 102;

pub fn main() {
    let detect = Detect::from_args(std::env::args())
        .unwrap_or_else(|_| {
            eprintln!("Did not get proper detect arguments");
            std::process::exit(ARGS_EXIT_CODE);
        })
        .unwrap_or_else(|| {
            eprintln!("Could not read user environment");
            std::process::exit(ARGS_EXIT_CODE);
        });

    if std::path::Path::new("./Cargo.toml").exists() {
        let mut build_plan = BuildPlan::new();
        build_plan.requires.push(Require::new("rust"));
        build_plan.requires.push(Require::new("cargo"));
        build_plan.provides.push(Provide::new("cargo"));

        detect
            .pass(Some(&build_plan))
            .unwrap_or_else(|_| detect.error(BUILDPLAN_EXIT_CODE));
    } else {
        detect.fail();
    }
}
