use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub title: String,
}

pub fn list_issues(repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/issues", repo);
    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-reqwest-client")
        .build()?;

    let response = client.get(url).send()?;
    let issues = response.json::<Vec<Issue>>()?;

    Ok(issues)
}
