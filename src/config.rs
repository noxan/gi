use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: ConfigGithub,
}

#[derive(Debug, Deserialize)]
pub struct ConfigGithub {
    pub token: String,
}
