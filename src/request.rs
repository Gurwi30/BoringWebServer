use anyhow::{anyhow, Result};
use std::collections::HashMap;

static METHODS: [&str; 2] = ["GET", "POST"];

#[derive(Debug)]
pub enum HeaderValue {
    Basic(String),
    List(Vec<String>),
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub protocol_version: String,
    pub headers: HashMap<String, HeaderValue>,
}

impl Request {
    pub fn new(method: String, path: String, protocol_version: String, headers: HashMap<String, HeaderValue>) -> Self {
        Request {
            method,
            path,
            protocol_version,
            headers
        }
    }

    pub fn get_header(&self, key: &str) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    pub fn parse(raw_request: &[u8]) -> Result<Request> {
        let (read_data, mut consumed_bytes) = read_until_eol(raw_request)
            .ok_or_else(|| anyhow!("Invalid HTTP request: No EOL found."))?;

        let data: String = String::from_utf8(read_data.to_vec())?;
        let values: Vec<&str> = data.split_whitespace().collect();

        if values.len() < 3 {
            return Err(anyhow!("Invalid HTTP request: Not enough values provided."));
        }

        if !METHODS.contains(&values[0]) {
            return Err(anyhow!("Invalid HTTP request: {} method is not allowed or invalid!", &values[0]));
        }

        let mut headers: HashMap<String, HeaderValue> = HashMap::new();

        while consumed_bytes < raw_request.len() {
            match read_until_eol(&raw_request[consumed_bytes..]) {
                Some((raw_data, length)) => {
                    let data: String = String::from_utf8(raw_data.to_vec())?;
                    let values: Vec<&str> = data.trim().split(":").collect();

                    consumed_bytes += length;

                    if values.len() < 2 {
                        continue;
                    }

                    let key = values[0];

                    let value: HeaderValue = if values[1].contains(',') && key != "User-Agent" {
                        let list: Vec<String> = values[1].split(',')
                            .map(|str| str.trim().to_string())
                            .collect();

                        HeaderValue::List(list)
                    } else {
                        HeaderValue::Basic(values[1][1..].to_string())
                    };

                    headers.insert(key.to_string(), value);
                }

                None => {}
            }
        }

        Ok(Request::new(values[0].into(), values[1][1..].into(), values[2].into(), headers))
    }
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