use crate::{Context, Result, anyhow};
use base64::prelude::*;
use regex::Regex;
use std::fs;

/// Read file to string but limit max size of file
/// max size: 5mb
pub fn safe_file_to_string(path: &str) -> Result<String> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("could not get metadata of file '{}'", path))?;

    // load 5mb max
    if metadata.len() > 5_000_000u64 {
        return Err(anyhow!("file size is too large to read into buffer"));
    }

    Ok(fs::read_to_string(path)?)
}

/// Creates embed marker
pub fn create_embed_marker(text: &str) -> String {
    format!("$%${}$%$", BASE64_STANDARD.encode(text))
}

/// Finds embed marker in text and returns it without markers
pub fn find_embed_marker(text: &str) -> Result<String> {
    let re = Regex::new(r"\$%\$(?P<base64>.+)\$%\$")?;
    let captures = re.captures(text)
        .context("failed to find embed data")?;

    Ok(captures["base64"].into())
}

pub fn read_embed_marker(text: &str) -> Result<String> {
    Ok(String::from_utf8(BASE64_STANDARD.decode(text)?)?)
}

