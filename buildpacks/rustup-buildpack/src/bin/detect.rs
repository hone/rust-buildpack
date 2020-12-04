use libcnb_rs::{
    build_plan::{BuildPlan, Provide},
    detect::Detect,
};

const ARGS_EXIT_CODE: i32 = 101;

fn main() {
    if let Some(detect) = Detect::from_args(std::env::args()) {
        let mut build_plan = BuildPlan::new();
        build_plan.provides.push(Provide::new("rust"));

        detect.pass();
    } else {
        eprintln!("Did not get proper detect arguments");
        std::process::exit(ARGS_EXIT_CODE);
    }
}
