use crate::cookie::Cookie;
use crate::encode::EncodeAlgo;
use crate::error::*;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::path::Path;

pub const CONTENT_TYPE: &[(&str, ContentType); 10] = &[
    ("text/html", ContentType::TextHtml),
    ("text/javascript", ContentType::TextJS),
    ("text/css", ContentType::TextCss),
    ("text/plain", ContentType::TextPlain),
    ("multipart/form-data", ContentType::MulForm),
    ("application/x-www-form-urlencoded", ContentType::AppForm),
    ("application/json", ContentType::AppJson),
    ("image/gif", ContentType::ImageGif),
    ("image/jpeg", ContentType::ImageJpeg),
    ("image/png", ContentType::ImagePng),
];

#[derive(Debug)]
pub struct HeadReq {
    pub method: Method,
    pub url: String,
    pub http_version: String,
    pub cookie: Cookie,
    pub host: String,
    pub user_agent: String,
    pub cont_len: Option<usize>,
    pub cont_type: Option<ContentType>,
    pub accept_encode: Option<Vec<EncodeAlgo>>,
    pub acr_headers: Option<Vec<String>>,
    pub acr_method: Option<String>,
    pub origin: Option<String>,
}

impl HeadReq {
    pub fn parse_from_utf8(v: &[u8]) -> NeroResult<Self> {
        let err = || NeroError::new_simple(NeroErrorKind::ParseHttpHeader);

        let mut head = Self::default();

        let string = String::from_utf8_lossy(v);
        let lines: Vec<&str> = string.split("\r\n").collect();

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
            .and_then(Self::parse_head_line)?;

        head.user_agent = lines
            .iter()
            .find(|line| line.starts_with("User-Agent"))
            .ok_or(err())
            .and_then(Self::parse_head_line)?;

        head.cookie = lines
            .iter()
            .find(|line| line.starts_with("Cookie"))
            .ok_or(err())
            .and_then(Self::parse_head_line)
            .map(Cookie::from_string)
            .unwrap_or(Cookie::default());

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

        head.accept_encode = lines
            .iter()
            .find(|line| line.starts_with("Accept-Encoding"))
            .and_then(|val| Self::parse_head_line(val).ok())
            .map(|val| val.split(", ").map(EncodeAlgo::parse_from_string).collect());

        head.acr_method = lines
            .iter()
            .find(|line| line.starts_with("Access-Control-Request-Method"))
            .and_then(|val| Self::parse_head_line(val).ok());

        head.acr_headers = lines
            .iter()
            .find(|line| line.starts_with("Access-Control-Request-Headers"))
            .and_then(|val| Self::parse_head_line(val).ok())
            .map(|val| val.split(", ").map(|s| s.to_string()).collect());

        head.origin = lines
            .iter()
            .find(|line| line.starts_with("Origin"))
            .and_then(|val| Self::parse_head_line(val).ok());

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

    pub fn is_acr(&self) -> bool {
        self.method == Method::Options
            && self.origin.is_some()
            && (self.acr_method.is_some() || self.acr_headers.is_some())
    }
}

impl Default for HeadReq {
    fn default() -> Self {
        Self {
            method: Method::Get,
            url: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookie: Cookie::default(),
            host: "".to_string(),
            user_agent: "".to_string(),
            cont_len: None,
            cont_type: None,
            accept_encode: None,
            acr_headers: None,
            acr_method: None,
            origin: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Options,
}

impl Method {
    pub fn parse_from_string<T: ToString>(string: T) -> NeroResult<Self> {
        match string.to_string().to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "options" => Ok(Self::Options),
            _ => Err(NeroError::new(
                NeroErrorKind::ParseHttpHeader,
                format!("Unknown method {}", string.to_string()),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    TextHtml,
    TextJS,
    TextCss,
    TextPlain,
    MulForm,
    AppForm,
    AppJson,
    ImageGif,
    ImageJpeg,
    ImagePng,
}

impl ContentType {
    pub fn parse_from_string<T: ToString>(string: T) -> Self {
        CONTENT_TYPE
            .iter()
            .find(|(s, _)| s == &string.to_string())
            .map(|(_, t)| t.clone())
            .unwrap_or(Self::AppForm)
    }

    pub fn format_to_string(&self) -> String {
        CONTENT_TYPE
            .iter()
            .find(|(_, t)| t == self)
            .map(|(s, _)| s.to_string())
            .unwrap_or("application/x-www-form-urlencoded".to_string())
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        match path.as_ref().extension() {
            Some(ext) => match ext.to_string_lossy().to_string().as_str() {
                "js" => Self::TextJS,
                "css" => Self::TextCss,
                "html" => Self::TextHtml,
                "txt" => Self::TextPlain,
                "gif" => Self::ImageGif,
                "png" => Self::ImagePng,
                "jpg" | "jpeg" => Self::ImageJpeg,
                _ => Self::AppForm,
            },
            None => Self::AppForm,
        }
    }
}

#[derive(Debug)]
pub struct HeadResp {
    pub http_version: String,
    pub status: Status,
    pub cont_type: ContentType,
    pub cont_len: usize,
    pub date: String,
    pub server: String,
    pub set_cookie: Option<Cookie>,
    pub cont_encode: Option<EncodeAlgo>,
    pub aca_origin: Option<String>,
    pub aca_methods: Option<Vec<String>>,
    pub aca_headers: Option<Vec<String>>,
}

impl HeadResp {
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

        if let Some(algo) = &self.cont_encode {
            res.push(format!("Content-Encoding: {}", algo.format_to_string()));
        }

        if let Some(origin) = &self.aca_origin {
            res.push(format!("Access-Control-Allow-Origin: {origin}"));
        }

        if let Some(headers) = &self.aca_headers {
            res.push(format!(
                "Access-Control-Allow-Headers: {}",
                headers.join(", ")
            ));
        }

        if let Some(methods) = &self.aca_methods {
            res.push(format!(
                "Access-Control-Allow-Methods: {}",
                methods.join(", ")
            ));
        }

        if let Some(cookie) = &self.set_cookie {
            res.push(format!("Set-Cookie: {}", cookie.format_to_string()));
        }

        res.push(String::new());
        res.push(String::new());

        res.join("\r\n")
    }
}

impl Default for HeadResp {
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
            set_cookie: None,
            cont_encode: None,
            aca_origin: None,
            aca_methods: None,
            aca_headers: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    Ok,
    NotFound,
    NoContent,
    Unauthorized,
    ServerError,
    BadRequest,
}

impl Status {
    pub fn status_info(&self) -> (u16, &'static str) {
        match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
            Self::NoContent => (204, "No Content"),
            Self::Unauthorized => (401, "Unauthorized"),
            Self::ServerError => (500, "Internal Server Error"),
            Self::BadRequest => (400, "Bad Request"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let info = self.status_info();

        f.write_fmt(format_args!("{}({})", info.0, info.1))
    }
}
