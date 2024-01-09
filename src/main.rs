mod cli;
mod config;
mod git;
mod github;

use crate::cli::Cli;
use crate::git::git_extract_owner_and_repo;
use crate::github::get_issue;
use config::read_config;
use github::list_issues;
use log::debug;
use slug::slugify;
use std::io;

fn cmd_list(access_token: String, owner: &str, repo: &str) {
    let issues = list_issues(access_token, owner, repo).expect("Could not list issues");

    for issue in &issues {
        println!("#{} {}", issue.number, issue.title)
    }
}

fn cmd_work(access_token: String, owner: &str, repo: &str, issue_number: &u64) {
    debug!(
        "Work on issue {} command for {}/{}",
        issue_number, owner, repo
    );

    let issue = get_issue(access_token, owner, repo, issue_number).expect("Could not get issue");
    debug!("The issue is {:?}", issue);

    let title_slug = slugify(issue.title);
    let branch_full_name = format!("{}-{}", issue_number, title_slug);

    // Limit branch name to 50 characters
    let branch_name = branch_full_name.chars().take(50).collect::<String>();
    println!("The branch name is {}", branch_name);

    git::create_and_checkout_branch(&branch_name).expect("Could not create branch");
}

fn cmd_pullrequest(access_token: String, owner: &str, repo: &str) {
    debug!("Pull request command for {}/{}", owner, repo);
}

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    // Read config file
    let config = read_config().expect("Could not read config");
    let access_token = config.github.token;
    debug!("The github access token is {}", access_token);

    // Retrieve project info
    let (owner, repo) = git_extract_owner_and_repo().expect("Could not get owner and repo");

    // Parse command line arguments
    let cli: Cli = argh::from_env();
    debug!("The cli is {:?}", cli);
    let issue_number = cli.issue;
    debug!("The issue is {:?}", issue_number);

    match cli.command {
        Some(command) => cmd_pullrequest(access_token, owner.as_str(), repo.as_str()),
        None => match issue_number {
            Some(issue_number) => {
                cmd_work(access_token, owner.as_str(), repo.as_str(), &issue_number)
            }
            None => cmd_list(access_token, owner.as_str(), repo.as_str()),
        },
    }

    Ok(())
}
