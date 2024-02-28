use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use serde::Deserialize;

pub fn setup_pipenv() -> Result<()> {
    // Install the 'pipenv' binary using the version defined in the `Pipfile`.
    install_pipenv_binary()?;

    // Use it to install other dependencies:
    install_project_packages()?;

    Ok(())
}

#[derive(Deserialize)]
struct Pipfile {
    packages: HashMap<String, String>,
}

fn install_pipenv_binary() -> Result<()> {
    let pip_file_toml = Path::repo_path("Pipfile").read_to_string()?;
    let pip_file: Pipfile = toml::from_str(&pip_file_toml)?;

    // This should be a value like "==YYYY.MM.DD"
    let version = pip_file
        .packages
        .get("pipenv")
        .context("Failed to find 'pipenv' in 'Pipfile' packages.")?;

    // pip3 install "pipenv==YYYY.MM.DD"
    Command::new("pip3")
        .arg("install")
        .arg(format!("pipenv{version}"))
        .run()
}

fn install_project_packages() -> Result<()> {
    let mut command = Command::new("python3")
        .property("-m", "pipenv")
        .arg("install");

    if GitHub::is_running_in_ci() {
        command = command.flag("--deploy");
    }

    command.run()
}
