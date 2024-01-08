use log;
use serde::Deserialize;
use std::{
    fs,
    io::{self, Read},
};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: ConfigGithub,
}

#[derive(Debug, Deserialize)]
pub struct ConfigGithub {
    pub token: String,
}

pub fn read_config() -> io::Result<Config> {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    log::debug!("The home directory is {}", home_dir.display());

    let config_path = home_dir.join(".config").join("gi.toml");
    log::debug!("The config path is {}", config_path.display());

    let mut file = fs::File::open(config_path).expect("Could not open config file");
    // TODO: create config file if it doesn't exist
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config = toml::from_str::<Config>(&contents).expect("Could not parse config file");
    log::debug!("The config is {:#?}", config);

    Ok(config)
}
