use std::{env, path::PathBuf, process::Command};

pub fn get_folder_and_json_name(test_name: &str) -> (String, String) {
    let values = test_name.split("::tests::").collect::<Vec<&str>>();
    let folder = values[0].to_string();
    let json_name = format!("{}{}", values[1].to_string(), ".json");
    (folder, json_name)
}


pub fn get_workspace_root() -> eyre::Result<String> {
    let current_dir = env::current_dir()?.into_os_string().into_string().map_err(|_|
        eyre::eyre!("Failed to convert current directory to string")
    )?;
    let cmd_output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()?;


    if !cmd_output.status.success() {
        return Ok(current_dir);
    }

    let json =
        serde_json::from_str::<serde_json::Value>(String::from_utf8(cmd_output.stdout)?.as_str())?;
    let path = match json.get("workspace_root") {
        Some(val) => match val.as_str() {
            Some(val) => val,
            None => return Ok(current_dir),
        },
        None => return Ok(current_dir),
    };

    Ok(String::from(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_workspace_root() {
        let workspace_root = get_workspace_root().unwrap();
        println!("workspace: {}", workspace_root);
        //assert_eq!(workspace_root, env::current_dir().unwrap().to_str().unwrap());
    }
}
