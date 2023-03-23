use anyhow::{bail, Result};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

use crate::Issue;
use crate::git::get_config_scoped;
use crate::utils::{ask_password, escape_branch_name};

const GITHUB_URL: &str = "https://api.github.com";

pub fn get(id: &str) -> Result<Box<dyn Issue>> {
    let title = get_issue_title(id)?;
    Ok(Box::new(GithubIssue { id: id.to_string(), title }))
}

struct GithubIssue {
    id: String,
    title: String,
}

impl Issue for GithubIssue {
    fn comment(&self) -> String {
        let comment = format!("{} - #{}", self.title, self.id);
        comment
    }

    fn branch(&self) -> String {
        let title = escape_branch_name(&self.title);
        let branch = format!("{}-{}", self.id, title);
        branch
    }
}

/// Fetch the issue from the server
fn get_issue_title(id: &str) -> Result<String> {
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

    let issue = response.json::<UpstreamContent>()?;

    Ok(issue.title)
}

/// The issue data
#[derive(Deserialize)]
struct UpstreamContent {
    title: String,
}

/// The failure data
#[derive(Deserialize)]
struct Failure {
    message: String,
}
