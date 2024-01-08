mod cli;
mod config;
mod git;
mod github;

use crate::git::extract_repo_from_remote_url;
use clap::{value_parser, Arg, Command};
use config::read_config;
use git::git_extract_remotes;
use github::list_issues;
use log::debug;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    let matches = Command::new("gi")
        .arg(
            Arg::new("issue")
                .value_parser(value_parser!(u64))
                .required(false),
        )
        .get_matches();
    let issue = matches.get_one::<u64>("issue");
    debug!("The issue is {:?}", issue);

    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    debug!("The github access token is {}", token);

    let git_remotes = git_extract_remotes()?;
    debug!(
        "The git remotes are {}",
        git_remotes
            .iter()
            .map(|(name, url)| format!("{} {}", name, url))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let remote = match git_remotes.get("origin") {
        Some(url) => url.to_string(),
        None => git_remotes
            .values()
            .next()
            .expect("Could not get remote url")
            .to_string(),
    };
    debug!("The remote is {}", remote);

    let (owner, repo) = extract_repo_from_remote_url(&remote)
        .expect("Could not extract owner and repo from remote url");
    debug!("The owner is {} and the repo is {}", owner, repo);

    let issues = list_issues(owner, repo).expect("Could not list issues");
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
