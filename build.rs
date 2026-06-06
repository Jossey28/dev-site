use std::process::Command;

fn main() { // Naive method to get current commit when running locally. TODO! Find alternatives 
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output().un;
}