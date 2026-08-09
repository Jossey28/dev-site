use std::process::Command;

fn main() {
    let cmd = Command::new("git").args(["rev-parse", "HEAD"]).output();
    if let Ok(val) = cmd {
        #[allow(clippy::unwrap_used)]
        let commit = String::from_utf8(val.stdout).unwrap();
        println!("cargo:rustc-env=GIT_COMMIT_SHA_RUST={commit}");
    }
}
