use reqwest;
use reqwest::header;
use serde::Deserialize;
use std::error::Error;

// Authentication
// https://docs.github.com/en/rest/authentication/authenticating-to-the-rest-api?apiVersion=2022-11-28

// https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
#[derive(Deserialize, Debug)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
}

fn request(
    access_token: String,
    url: String,
) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("token {}", access_token).as_str())?,
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-reqwest-client")
        .default_headers(headers)
        .build()?;

    let response = client.get(url).send()?;
    Ok(response)
}

pub fn list_issues(
    access_token: String,
    owner: &str,
    repo: &str,
) -> Result<Vec<Issue>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?filter=-is:pr",
        owner, repo
    );
    let response = request(access_token, url)?;
    let issues = response.json::<Vec<Issue>>()?;

    Ok(issues)
}

pub fn get_issue(
    access_token: String,
    owner: &str,
    repo: &str,
    issue_number: &u64,
) -> Result<Issue, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}",
        owner, repo, issue_number
    );
    let response = request(access_token, url)?;
    let issue = response.json::<Issue>()?;

    Ok(issue)
}
