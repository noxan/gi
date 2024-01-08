mod config;
use std::{env, io};

use config::read_config;

fn main() -> io::Result<()> {
    env_logger::init();

    println!("Hello, world!");

    let config = read_config().expect("Could not read config");
    let token = config.github.token;
    println!("The github access token is {}", token);

    let current_dir = env::current_dir()?;
    println!("The current directory is {}", current_dir.display());

    Ok(())
}
