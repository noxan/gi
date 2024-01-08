mod config;
use config::read_config;
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

    Ok(())
}
