use libcnb_rs::{
    build_plan::{BuildPlan, Provide},
    detect::Detect,
};

const ARGS_EXIT_CODE: i32 = 101;
const BUILDPLAN_EXIT_CODE: i32 = 102;

fn main() {
    if let Ok(Some(detect)) = Detect::from_args(std::env::args()) {
        let mut build_plan = BuildPlan::new();
        build_plan.provides.push(Provide::new("rust"));

        detect
            .pass(Some(&build_plan))
            .unwrap_or_else(|_| detect.error(BUILDPLAN_EXIT_CODE));
    } else {
        eprintln!("Did not get proper detect arguments");
        std::process::exit(ARGS_EXIT_CODE);
    }
}
