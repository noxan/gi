mod cli;
mod config;
mod git;
mod github;

use crate::git::git_extract_owner_and_repo;
use config::read_config;
use github::list_issues;
use log::debug;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    // Read config file
    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    debug!("The github access token is {}", token);

    // Parse command line arguments
    let matches = cli::parse();
    let issue_number = matches.get_one::<u64>("issue");
    debug!("The issue is {:?}", issue_number);

    let (owner, repo) = git_extract_owner_and_repo().expect("Could not get owner and repo");

    let issues = list_issues(owner.as_str(), repo.as_str()).expect("Could not list issues");
    debug!(
        "The issues are \n{}",
        issues
            .iter()
            .map(|issue| format!("- {} (#{})", issue.title, issue.number))
            .collect::<Vec<String>>()
            .join("\n")
    );

    Ok(())
}
