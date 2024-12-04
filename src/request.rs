use anyhow::{anyhow, Result};
use std::collections::HashMap;

static METHODS: [&str; 2] = ["GET", "POST"];

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub protocol_version: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(method: String, path: String, protocol_version: String) -> Self {
        Request {
            method,
            path,
            protocol_version,
            headers: HashMap::new(),
        }
    }


}

pub fn parse(raw_request: &[u8]) -> Result<Request> {
    let (read_data, _consumed_bytes) = read_until_eol(raw_request)
        .ok_or_else(|| anyhow!("Invalid HTTP request: No EOL found."))?;

    let data: String = String::from_utf8(read_data.to_vec())?;
    let values: Vec<&str> = data.split_whitespace().collect();

    if values.len() < 3 {
        return Err(anyhow!("Invalid HTTP request: Not enough values provided."));
    }

    if !METHODS.contains(&values[0]) {
        return Err(anyhow!("Invalid HTTP request: {} method is not allowed or invalid!", &values[0]));
    }

    Ok(Request::new(values[0].into(), values[1][1..].into(), values[2].into()))
}

fn read_until_eol(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        let previous_char = buffer[i - 1] as char;
        let cur_char: char = buffer[i] as char;

        if cur_char == '\n' && previous_char == '\r' {
            return Some((&buffer[0..(i - 1)], i + 1));
        }
    }

    None
}