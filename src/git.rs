use std::io::{self, Write};
use std::process::Command;

pub fn git_commit(message: &String, add_all_files: bool) {
    let mut git = Command::new("git");

    git.arg("commit").arg("--message").arg(message);

    if add_all_files {
        git.arg("-a");
    }

    let output = git.output().expect("failed to execute git process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
