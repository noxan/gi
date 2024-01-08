mod config;
mod git;
mod github;

use config::read_config;
use git::git_extract_remotes;
use github::list_issues;
use log::debug;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

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

    let issues = list_issues("rust-lang/rust").expect("Could not list issues");
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
