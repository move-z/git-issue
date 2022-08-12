use anyhow::{bail, Result};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

use crate::utils::{ask_password, get_config_scoped};

const GITHUB_URL: &str = "https://api.github.com";

/// Fetch the issue from the server
pub fn get_issue_title(id: &str) -> Result<String> {
    let user = get_config_scoped("user", "github")?;
    let repo = get_config_scoped("repo", "github")?;
    let public = if let Ok(public) = get_config_scoped("public", "github") {
        match public.as_str() {
            "true" => true,
            "false" => false,
            _ => bail!("option public must be true or false"),
        }
    } else {
        false
    };

    println!("Fetching info on github issue {id}");

    let url = format!("{GITHUB_URL}/repos/{user}/{repo}/issues/{id}");
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header(ACCEPT, "application/vnd.github+json")
        .header(USER_AGENT, "git-issue");
    let response = if !public {
        let password = ask_password(&user)?;
        response.basic_auth(user, Some(password))
    } else {
        response
    };
    let response = response.send()?;

    let status = response.status();
    if !status.is_success() {
        if let Ok(f) = response.json::<Failure>() {
            let message = f.message;
            bail!("received error from github server:\n{message}");
        } else {
            bail!("received error code from jira server:\n{status}");
        }
    }

    let issue = response.json::<Issue>()?;

    Ok(issue.title)
}

/// The issue data
#[derive(Deserialize)]
struct Issue {
    title: String,
}

/// The failure data
#[derive(Deserialize)]
struct Failure {
    message: String,
}
