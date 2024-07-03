use anyhow::Result;
use regex::Regex;

/// Ask for password
pub fn ask_password(prompt: &str) -> Result<String> {
    let p = rpassword::prompt_password(format!("Enter password for {prompt}: "))?;
    Ok(p)
}

pub fn escape_branch_name(input: &str) -> String {
    let res = Regex::new(r"[\p{Space Separator}/]")
        .unwrap()
        .replace_all(input, "-");
    // partially implemented according to
    // https://git-scm.com/docs/git-check-ref-format
    let res = Regex::new(r"[\p{Control},~^:?*\[\]\\@{}]")
        .unwrap()
        .replace_all(&res, "");
    let res = Regex::new(r"\.\.+").unwrap().replace_all(&res, ".");
    let res = Regex::new(r"--+").unwrap().replace_all(&res, "-");
    res.into_owned()
}
