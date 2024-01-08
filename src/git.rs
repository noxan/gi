use configparser::ini::Ini;
use log::debug;
use std::{collections::HashMap, env, fs, io, path::PathBuf};

fn git_config_path() -> io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    debug!("The current directory is {}", current_dir.display());

    let git_config_path = current_dir.join(".git").join("config");
    debug!("The git config path is {}", git_config_path.display());

    Ok(git_config_path)
}

pub fn git_extract_remotes() -> io::Result<HashMap<String, String>> {
    let git_config_path = git_config_path().expect("Could not get git config path");

    let mut git_config = Ini::new();

    let contents = fs::read_to_string(git_config_path)?;
    git_config
        .read(contents)
        .expect("Could not parse git config");

    let mut remotes = HashMap::new();

    let sections = git_config.sections();
    debug!("The sections are [{}]", sections.join(", "));

    for section in sections {
        if section.starts_with("remote ") {
            let remote_name = section.trim_start_matches("remote ").replace("\"", "");
            debug!("The remote name is {}", remote_name);

            let remote_url = git_config
                .get(&section, "url")
                .expect("Could not get remote url");
            debug!("The remote url is {}", remote_url);

            remotes.insert(remote_name.to_string(), remote_url.to_string());
        }
    }

    Ok(remotes)
}
