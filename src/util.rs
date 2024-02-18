use crate::{Context, Result};
use base64::prelude::*;
use regex::Regex;

const EMBED_MARKER: &'static str = "$%$";

pub fn create_embed_marker(text: &str) -> String {
    format!("{0}{1}{0}", EMBED_MARKER, BASE64_STANDARD.encode(text))
}

/// Finds embed marker in text and returns it without markers
pub fn find_embed_marker(text: &str) -> Result<String> {
    let re = Regex::new(format!(r"{0}(?P<base64>{0}){0}", EMBED_MARKER).as_str())?;
    let captures = re.captures(text)
        .context("failed to find embed data")?;

    Ok(captures["base64"].into())
}

pub fn read_embed_marker(text: &str) -> Result<String> {
    Ok(String::from_utf8(BASE64_STANDARD.decode(text)?)?)
}

