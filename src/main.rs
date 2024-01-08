mod cli;
mod config;
mod git;
mod github;

use crate::git::git_extract_owner_and_repo;
use config::read_config;
use github::list_issues;
use log::debug;
use std::io;

fn cmd_list(owner: &str, repo: &str) {
    let issues = list_issues(owner, repo).expect("Could not list issues");

    for issue in &issues {
        println!("#{} {}", issue.number, issue.title)
    }
}

fn cmd_work(owner: &str, repo: &str, issue_number: &u64) {
    println!(
        "Work on issue {} command for {}/{}",
        issue_number, owner, repo
    );
}

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    // Read config file
    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    debug!("The github access token is {}", token);

    // Retrieve project info
    let (owner, repo) = git_extract_owner_and_repo().expect("Could not get owner and repo");

    // Parse command line arguments
    let matches = cli::parse();
    let issue_number = matches.get_one::<u64>("issue");
    debug!("The issue is {:?}", issue_number);

    if issue_number.is_none() {
        cmd_list(owner.as_str(), repo.as_str());
    } else {
        cmd_work(owner.as_str(), repo.as_str(), issue_number.unwrap());
    }

    Ok(())
}
