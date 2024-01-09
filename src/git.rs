use configparser::ini::Ini;
use git2::{BranchType, Repository};
use log::debug;
use std::error::Error;
use std::{collections::HashMap, env, fs, io, path::PathBuf};

fn get_repo_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Could not get current directory");
    debug!("The current directory is {}", current_dir.display());

    current_dir
}

fn git_config_path() -> io::Result<PathBuf> {
    let repo_path = get_repo_path();

    let git_config_path = repo_path.join(".git").join("config");
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

pub fn extract_repo_from_remote_url(remote_url: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = if remote_url.contains("https://") {
        remote_url.trim_end_matches(".git").split('/').collect()
    } else {
        remote_url.trim_end_matches(".git").split(':').collect()
    };

    if parts.len() >= 2 {
        let repo_parts = parts.last().unwrap().split('/').collect::<Vec<&str>>();
        if repo_parts.len() == 2 {
            return Some((repo_parts[0], repo_parts[1]));
        }
    }

    None
}

pub fn git_extract_owner_and_repo() -> io::Result<(String, String)> {
    let git_remotes = git_extract_remotes()?;
    debug!(
        "The git remotes are {}",
        git_remotes
            .iter()
            .map(|(name, url)| format!("{} {}", name, url))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let remote = match git_remotes.get("origin") {
        Some(url) => url.to_string(),
        None => git_remotes
            .values()
            .next()
            .expect("Could not get remote url")
            .to_string(),
    };
    debug!("The remote is {}", remote);

    let (owner, repo) = extract_repo_from_remote_url(&remote)
        .expect("Could not extract owner and repo from remote url");
    debug!("The owner is {} and the repo is {}", owner, repo);

    Ok((owner.to_string(), repo.to_string()))
}

pub fn create_and_checkout_branch(branch_name: &str) -> Result<(), Box<dyn Error>> {
    let repo_path = get_repo_path();

    let repo = Repository::open(repo_path)?;
    let remote_branch = repo.find_branch("origin/main", BranchType::Remote)?;
    let target = remote_branch.get().peel_to_commit()?;
    let new_branch = repo.branch(branch_name, &target, false)?;

    repo.set_head(new_branch.get().name().unwrap())?;

    Ok(())
}
