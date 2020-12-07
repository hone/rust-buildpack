use libcnb_rs::build::Build;

pub fn main() {
    Build::from_args(std::env::args());
}
