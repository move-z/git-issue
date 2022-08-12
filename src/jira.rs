use crate::git;
use anyhow::Result;

/// Handle the request for this jira id
pub fn handle(id: &str) -> Result<()> {
    let host = git::get_config("jira.host")?;
    let username = git::get_config("jira.user")?;

    let _issue = fetch_jira_issue(&host, &username, id);
    todo!()
}

/// Cetch the issue from the server
fn fetch_jira_issue(host: &str, username: &str, id: &str) -> Issue {
    println!("Fetching info on jira issue {id}");

    todo!()
}

/// The issue data
struct Issue {
    id: String,
    title: String,
}
