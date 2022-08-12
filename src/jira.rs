use crate::git;
use anyhow::{bail, Result};
use serde::Deserialize;

/// Handle the request for this jira id
pub fn handle(id: &str) -> Result<()> {
    let host = git::get_config("jira.host")?;
    let username = git::get_config("jira.user")?;

    let _issue = fetch_jira_issue(&host, &username, id)?;
    todo!()
}

/// Cetch the issue from the server
fn fetch_jira_issue(host: &str, username: &str, id: &str) -> Result<Issue> {
    println!("Fetching info on jira issue {id}");

    let password = "aaa";

    let url = format!("{host}/rest/api/latest/issue/{id}");
    let response = reqwest::blocking::Client::new()
        .get(url)
        .basic_auth(username, Some(password))
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
    // .json::<Issue>()?;

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
