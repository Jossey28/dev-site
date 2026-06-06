use std::process::Command;

fn main() { // Naive method to get current commit when running locally. TODO! Find alternatives 
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
    match output {
        Ok(out) => {
            let commit = String::from_utf8(out.stdout).unwrap_or(String::from("Unable to covert command result to String"));
            println!("cargo:rustc-env=GIT_COMMIT_SHA_RUST={}", commit);
        }

        Err(err) => {
            println!("Unable to get command result; Error: {}", err);
            return
        }
    }

}