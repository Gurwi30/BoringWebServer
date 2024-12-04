use anyhow::Result;
use std::fs;

pub enum ContentType {
    Text,
    Image
}

impl ContentType {
    fn get_prefix(&self) -> &str {
        match self {
            ContentType::Text => "text/",
            ContentType::Image => "image/"
        }
    }
}

pub enum Status {
    Success,
    NotFound
}

impl Status {
    fn get_header(&self) -> &str {
        match self {
            Status::Success => "200 OK",
            Status::NotFound => "404 Not Found"
        }
    }
}

pub fn create_resp(status: Status, content_type: ContentType, content: &str, file_path: &str) -> Result<(String, Vec<u8>)> {
    let mut data: Vec<Vec<u8>> = Vec::new();
    let contents = fs::read(file_path)?;
    let len = contents.len();
    let mut resp = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}{}\r\nServer: BoringWebServer 0.1.0\r\n\r\n", status.get_header(), len, content_type.get_prefix(), content);

    data.push(resp.as_bytes().to_vec());

    Ok((resp, contents))
}

pub fn create_basic_html_resp(status: Status, html_content: &str) -> String {
    format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}{}\r\nServer: BoringWebServer 0.1.0\r\n\r\n{}", status.get_header(), html_content.len(), ContentType::Text.get_prefix(), "html", html_content)
}

pub fn get_content_type(file_path: &str) -> (ContentType, &str) {
    match file_path.to_lowercase().split('.').last() {
        Some(ext) => {
            match ext {
                "html" => (ContentType::Text, "html"),
                "css" => (ContentType::Text, "css"),
                "js" => (ContentType::Text, "javascript"),
                "json" => (ContentType::Text, "json"),
                "png" => (ContentType::Image, "png"),
                "svg" => (ContentType::Image, "svg+xml"),
                "gif" => (ContentType::Image, "gif"),
                "ico" => (ContentType::Image, "vnd.microsoft.icon"),
                _ => (ContentType::Text, "plain")
            }
        },

        None => (ContentType::Text, "plain")
    }
}