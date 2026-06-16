use std::{env, process::Command};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed={crate_dir}/git/HEAD");

    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);

            println!("cargo:rustc-env=GIT_BUILD_REV={git_hash}");
        }
    }
}
