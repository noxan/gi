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

fn request(url: String) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-reqwest-client")
        .build()?;

    let response = client.get(url).send()?;
    Ok(response)
}

pub fn list_issues(owner: &str, repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let response = request(url)?;
    let issues = response.json::<Vec<Issue>>()?;

    Ok(issues)
}

pub fn get_issue(owner: &str, repo: &str, issue_number: &u64) -> Result<Issue, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}",
        owner, repo, issue_number
    );
    let response = request(url)?;
    let issue = response.json::<Issue>()?;

    Ok(issue)
}
