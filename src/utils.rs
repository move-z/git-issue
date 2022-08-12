use anyhow::Result;

/// Ask for password
pub fn ask_password(prompt: &str) -> Result<String> {
    let p = rpassword::prompt_password(format!("Enter password for {prompt}: "))?;
    Ok(p)
}
