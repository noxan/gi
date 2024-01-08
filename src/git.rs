use configparser::ini::Ini;
use log::debug;
use std::{env, fs, io, path::PathBuf};

fn git_config_path() -> io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    debug!("The current directory is {}", current_dir.display());

    let git_config_path = current_dir.join(".git").join("config");
    debug!("The git config path is {}", git_config_path.display());

    Ok(git_config_path)
}

pub fn git_extract_remotes() -> io::Result<()> {
    let git_config_path = git_config_path().expect("Could not get git config path");

    let mut git_config = Ini::new();

    let contents = fs::read_to_string(git_config_path)?;
    git_config
        .read(contents)
        .expect("Could not parse git config");

    let sections = git_config.sections();
    debug!("The sections are {:?}", sections);

    let remotes = git_config
        .sections()
        .iter()
        .filter(|section| section.starts_with("remote "))
        .map(|section| section.trim_start_matches("remote ").to_string())
        .collect::<Vec<String>>();

    debug!("The remotes are {:?}", remotes);

    Ok(())
}
