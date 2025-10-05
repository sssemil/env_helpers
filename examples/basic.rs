use env_helpers::get_env;
use std::path::PathBuf;

fn main() {
    let home_path: PathBuf = get_env("HOME");
    println!("{:?}", home_path);
}
