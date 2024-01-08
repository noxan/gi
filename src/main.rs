mod config;
mod git;
use config::read_config;
use git::git_extract_remotes;
use log::debug;
use std::{env, io};

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    debug!("The github access token is {}", token);

    let current_dir = env::current_dir()?;
    debug!("The current directory is {}", current_dir.display());

    // let git_config = read_git_config()?;
    // debug!("The git config is {}", git_config);
    git_extract_remotes()?;

    Ok(())
}
