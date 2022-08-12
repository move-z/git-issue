use std::io::{self, Write};

use crate::git;
use anyhow::{bail, Result};
use serde::Deserialize;

/// Handle the request for this jira id
pub fn handle(id: &str) -> Result<()> {
    let host = git::get_config("jira.host")?;
    let user = git::get_config("jira.user")?;

    let _issue = fetch_jira_issue(&host, &user, id)?;
    todo!()
}

/// Cetch the issue from the server
fn fetch_jira_issue(host: &str, user: &str, id: &str) -> Result<Issue> {
    println!("Fetching info on jira issue {id}");

    let password = ask_password(user, host)?;

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

/// Ask for password
fn ask_password(user: &str, host: &str) -> Result<String> {
    print!("Enter password for user {user} on server {host}: ");
    io::stdout().flush()?;

    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    Ok(password)
}

