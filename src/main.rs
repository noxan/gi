mod config;
mod git;
use config::read_config;
use git::git_extract_remotes;
use log::debug;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    debug!("Hello, world!");

    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    debug!("The github access token is {}", token);

    let git_remotes = git_extract_remotes()?;
    debug!("The git remotes are {:?}", git_remotes);

    Ok(())
}
