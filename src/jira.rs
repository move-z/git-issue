use anyhow::{bail, Result};
use serde::Deserialize;

use crate::git::get_config_scoped;
use crate::utils::ask_password;

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

    let issue = response.json::<Issue>()?;

    Ok(issue.fields.summary)
}

/// The issue data
#[derive(Deserialize)]
struct Issue {
    fields: Fields,
}

/// The json fields
#[derive(Deserialize)]
struct Fields {
    summary: String,
}

/// The error data
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Failure {
    errorMessages: Vec<String>,
}
