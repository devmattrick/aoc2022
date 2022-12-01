use anyhow::Result;
use std::fs;

pub fn read_input(day: u32) -> Result<String> {
    let file = format!("input/input{:02}.txt", day);
    let raw = fs::read_to_string(file)?;

    Ok(format_input(&raw).to_owned())
}

pub fn format_input(raw: &str) -> &str {
    raw.trim()
}
