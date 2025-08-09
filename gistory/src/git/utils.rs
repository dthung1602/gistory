use std::process;

use crate::error;

pub fn get_global_config(config_name: &str) -> error::Result<String> {
    let output = process::Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg(config_name)
        .output()?;
    if !output.status.success() {
        return Err(error::Error::Command(format!(
            "Cannot get {config_name} from global git config."
        )));
    }
    let value = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(value)
}
