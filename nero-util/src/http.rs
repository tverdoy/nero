use crate::cookie::Cookie;
use crate::error::*;

#[derive(Debug)]
pub struct HttpHeadReq {
    method: Method,
    url: String,
    http_version: String,
    cookie: Cookie,
    host: String,
    user_agent: String,
    cont_len: Option<usize>,
    cont_type: Option<ContentType>
}

impl HttpHeadReq {
    pub fn parse_from_utf8(v: &[u8]) -> Result<Self> {
        let err = || Error::new_simple(ErrorKind::ParseHttpHeader);

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
    pub fn parse_head_line<T: ToString>(line: T) -> Result<String> {
        let line = line.to_string();
        let split: Vec<&str> = line.split(": ").collect();
        if split.len() == 2 {
            Ok(split[1].to_string())
        } else {
            Err(Error::new_simple(ErrorKind::ParseHttpHeader))
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
            cont_type: Some(ContentType::Other("".to_string()))
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse_from_string<T: ToString>(string: T) -> Result<Self> {
        match string.to_string().to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            _ => Err(Error::new(
                ErrorKind::ParseHttpHeader,
                format!("Unknown method"),
            )),
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextHtml,
    FormData,
    FormUrlencoded,
    Other(String)
}

impl ContentType {
    pub fn parse_from_string<T: ToString>(string: T) -> Self {
        match string.to_string().to_lowercase().as_str() {
            "text/html" => Self::TextHtml,
            "multipart/form-data" => Self::FormData,
            "application/x-www-form-urlencoded" => Self::FormUrlencoded,
            _ => Self::Other(string.to_string()),
        }
    }
}
