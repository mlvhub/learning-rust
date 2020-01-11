extern crate quicli;
use quicli::prelude::*;

extern crate structopt;
use structopt::StructOpt;

use std::process::Command;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long = "repository-dir", short = "r")]
    repo_dir: String,
    #[structopt(long = "base-branch", short = "b")]
    base_branch: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    let repo_dir = args.repo_dir;
    let base_branch = args.base_branch;

    println!("starting from repo {} and branch {}", repo_dir, base_branch);

    let branches_output = Command::new("/usr/local/bin/git")
        .arg("branch")
        .current_dir(repo_dir)
        .output()
        .expect("failed to list branches");

    println!("branch list: {}", branches_output.status);
    println!("branch list: {}", String::from_utf8_lossy(&branches_output.stdout));

    let str_output = String::from_utf8(branches_output.stdout).expect("Error getting the branches.");
    let branches: std::slice::Iter<&str> = str_output.split("\n").collect::<Vec<&str>>().iter();

    let branches_to_update = branches.skip_while(|b: &&str| (*b).contains(base_branch));

    branches_to_update.for_each(|s| println!("{}", s));

    Ok(())
}
