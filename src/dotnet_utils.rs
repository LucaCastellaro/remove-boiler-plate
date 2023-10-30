use crate::constants;

use std::{process::{Command, Output}, collections::HashSet, io::Error};

pub fn publish(publish_mod: &String) -> Result<Output, Error> {
    let deploy_mod: String = if publish_mod == constants::RELEASE { constants::RELEASE.to_string() } else { constants::DEBUG.to_string() };
    return Command::new("dotnet")
        .arg("publish")
        .arg(format!("-p:Configuration={}", deploy_mod))
        .output();
}

pub fn get_versions() -> Result<HashSet<String>, String> {
    let output = Command::new("dotnet")
        .arg("--list-sdks")
        .output();
    
    if output.is_err() {
        return Err(output.err().unwrap().to_string());
    }
    
    let stdout = String::from_utf8(output.unwrap().stdout);
    if stdout.is_err() {
        return Err(stdout.err().unwrap().to_string());
    }
    
    let result = stdout.unwrap();
    let result: HashSet<String> = result.split('\n')
        .filter(|x| x.trim().len() > 0)
        .map(format_versions)
        .collect();
    
    return Ok(result);
}

fn format_versions(source: &str) -> String {
    if source.len() < 2 {
        return source.to_string();
    }
    
    let mut iter = source.char_indices();
    let (start, _) = iter.nth(0).unwrap();
    let (end, _) = iter.nth(2).unwrap();
    let slice = &source[start..end];
    return slice.to_string();
}