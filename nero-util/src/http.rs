use crate::cookie::Cookie;
use crate::error::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

pub const CONTENT_TYPE: &[(&str, ContentType); 6] = &[
    ("text/html", ContentType::TextHtml),
    ("text/javascript", ContentType::TextJS),
    ("text/css", ContentType::TextCss),
    ("multipart/form-data", ContentType::MulForm),
    ("application/x-www-form-urlencoded", ContentType::AppForm),
    ("application/json", ContentType::AppJson),
];

#[derive(Debug)]
pub struct HttpHeadReq {
    pub method: Method,
    pub url: String,
    pub http_version: String,
    pub cookie: Cookie,
    pub host: String,
    pub user_agent: String,
    pub cont_len: Option<usize>,
    pub cont_type: Option<ContentType>,
}

impl HttpHeadReq {
    pub fn parse_from_utf8(v: &[u8]) -> NeroResult<Self> {
        let err = || NeroError::new_simple(NeroErrorKind::ParseHttpHeader);

        let mut head = Self::default();

        let string = String::from_utf8_lossy(v);
        let mut lines: Vec<&str> = string.split("\r\n").collect();

        match lines.first() {
            Some(first) => {
                let split: Vec<&str> = first.split_ascii_whitespace().collect();
                if split.len() != 3 {
                    Err(err())?
                }

                head.method = Method::parse_from_string(split[0])?;
                head.url = split[1].to_string();
                head.http_version = split[2].to_string();
            }
            None => Err(err())?,
        };

        head.host = lines
            .iter()
            .find(|line| line.starts_with("Host"))
            .ok_or(err())
            .and_then(Self::parse_head_line)?
            .to_string();

        head.user_agent = lines
            .iter()
            .find(|line| line.starts_with("User-Agent"))
            .ok_or(err())
            .and_then(Self::parse_head_line)?
            .to_string();

        head.cookie = lines
            .iter()
            .find(|line| line.starts_with("User-Agent"))
            .ok_or(err())
            .and_then(Self::parse_head_line)
            .map(Cookie::from_string)?;

        head.cont_len = lines
            .iter()
            .find(|line| line.starts_with("Content-Length"))
            .and_then(|val| Self::parse_head_line(val).ok())
            .and_then(|val| val.parse::<usize>().ok());

        head.cont_type = lines
            .iter()
            .find(|line| line.starts_with("Content-Type"))
            .and_then(|val| Self::parse_head_line(val).ok())
            .map(ContentType::parse_from_string);

        Ok(head)
    }

    /// Return value of head line
    pub fn parse_head_line<T: ToString>(line: T) -> NeroResult<String> {
        let line = line.to_string();
        let split: Vec<&str> = line.split(": ").collect();
        if split.len() == 2 {
            Ok(split[1].to_string())
        } else {
            Err(NeroError::new_simple(NeroErrorKind::ParseHttpHeader))
        }
    }
}

impl Default for HttpHeadReq {
    fn default() -> Self {
        Self {
            method: Method::Get,
            url: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookie: Cookie::new(),
            host: "".to_string(),
            user_agent: "".to_string(),
            cont_len: Some(0),
            cont_type: Some(ContentType::AppForm),
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse_from_string<T: ToString>(string: T) -> NeroResult<Self> {
        match string.to_string().to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            _ => Err(NeroError::new(
                NeroErrorKind::ParseHttpHeader,
                format!("Unknown method"),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    TextHtml,
    TextJS,
    TextCss,
    MulForm,
    AppForm,
    AppJson,
}

impl ContentType {
    pub fn parse_from_string<T: ToString>(string: T) -> Self {
        CONTENT_TYPE
            .iter()
            .find(|(s, t)| s == &string.to_string())
            .map(|(s, t)| t.clone())
            .unwrap_or(Self::AppForm)
    }

    pub fn format_to_string(&self) -> String {
        CONTENT_TYPE
            .iter()
            .find(|(s, t)| t == self)
            .map(|(s, t)| s.to_string())
            .unwrap_or("application/x-www-form-urlencoded".to_string())

    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        match path.as_ref().extension() {
            Some(ext) => {
                match ext.to_string_lossy().to_string().as_str() {
                    "js" => Self::TextJS,
                    "css" => Self::TextCss,
                    "html" => Self::TextHtml,
                    _ => Self::AppForm
                }
            },
            None => Self::AppForm
        }
    }
}

pub struct HttpHeadResp {
    pub http_version: String,
    pub status: Status,
    pub cont_type: ContentType,
    pub cont_len: usize,
    pub date: String,
    pub server: String,
}

impl HttpHeadResp {
    pub fn format_to_string(&self) -> String {
        let mut res = Vec::new();

        let (status_code, status_text) = self.status.status_info();
        res.push(format!(
            "{} {} {}",
            self.http_version, status_code, status_text
        ));
        res.push(format!("Server: {}", self.server));
        res.push(format!("Date: {}", self.date));
        res.push(format!(
            "Content-Type: {}",
            self.cont_type.format_to_string()
        ));
        res.push(format!("Content-Length: {}", self.cont_len));
        res.push(String::new());
        res.push(String::new());

        res.join("\r\n")
    }
}

impl Default for HttpHeadResp {
    fn default() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let date = format!("{}", utc.format("%a, %d %b %Y %T GMT"));

        Self {
            http_version: "HTTP/1.0".to_string(),
            status: Status::Ok,
            cont_type: ContentType::TextHtml,
            cont_len: 0,
            date,
            server: "Nero".to_string(),
        }
    }
}

pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn status_info(&self) -> (u16, &'static str) {
        match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
        }
    }
}
