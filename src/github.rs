use reqwest;
use serde::Deserialize;
use std::error::Error;

// https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
#[derive(Deserialize, Debug)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
}

pub fn list_issues(owner: &str, repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-reqwest-client")
        .build()?;

    let response = client.get(url).send()?;
    let issues = response.json::<Vec<Issue>>()?;

    Ok(issues)
}
