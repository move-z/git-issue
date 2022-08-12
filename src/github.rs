use anyhow::{bail, Result};
use reqwest::header::ACCEPT;

use crate::utils::{get_config_scoped, ask_password};

const GITHUB_URL: &'static str = "https://api.github.com";

/// Fetch the issue from the server
pub fn get_issue_title(id: &str) -> Result<String> {
    let user = get_config_scoped("user", "github")?;
    let repo = get_config_scoped("repo", "github")?;

    println!("Fetching info on github issue {id}");

    let password = ask_password(&user)?;

    let url = format!("{GITHUB_URL}/repos/{user}/{repo}/issues/{id}");
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header(ACCEPT, "application/vnd.github+json")
        .basic_auth(user, Some(password))
        .send()?;

    dbg!(response);
    // let status = response.status();
    // if !status.is_success() {
    //     if let Ok(f) = response.json::<Failure>() {
    //         let errors = f.errorMessages.join("\n");
    //         bail!("received errors from jira server:\n{errors}");
    //     } else {
    //         bail!("received error code from jira server:\n{status}");
    //     }
    // }
    todo!()
}
