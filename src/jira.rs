use anyhow::{bail, Result};
use serde::Deserialize;

use crate::utils::{ask_password, get_config_scoped};

/// Fetch the issue from the server
pub fn get_issue_title(id: &str) -> Result<String> {
    let host = get_config_scoped("host", "jira")?;
    let user = get_config_scoped("user", "jira")?;

    println!("Fetching info on jira issue {id}");

    let password = ask_password(&format!("user {user} on host {host}"))?;

    let url = format!("{host}/rest/api/latest/issue/{id}");
    let response = reqwest::blocking::Client::new()
        .get(url)
        .basic_auth(user, Some(password))
        .send()?;

    let status = response.status();
    if !status.is_success() {
        if let Ok(f) = response.json::<Failure>() {
            let errors = f.errorMessages.join("\n");
            bail!("received errors from jira server:\n{errors}");
        } else {
            bail!("received error code from jira server:\n{status}");
        }
    }

    dbg!(response);
    todo!()
}

/// The issue data
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Issue {}

/// The error data
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Failure {
    errorMessages: Vec<String>,
}
