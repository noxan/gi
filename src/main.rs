use std::{
    env, fs,
    io::{self, Read},
};
mod config;
use config::Config;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    println!("The home directory is {}", home_dir.display());

    let config_path = home_dir.join(".config").join("gi.toml");
    println!("The config path is {}", config_path.display());

    let mut file = fs::File::open(config_path).expect("Could not open config file");
    // TODO: create config file if it doesn't exist
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config = toml::from_str::<Config>(&contents).expect("Could not parse config file");
    println!("The config is {:#?}", config);

    let token = config.github.token;
    println!("The github access token is {}", token);

    let current_dir = env::current_dir()?;
    println!("The current directory is {}", current_dir.display());

    Ok(())
}
