use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    RequestError(String),
    ServerError(StatusCode, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ParseError(err) => format!("parse error: {}", err),
            Self::RequestError(err) => format!("request error: {}", err),
            Self::ServerError(code, msg) => format!("[{}] {}", code, msg),
        };

        write!(f, "{}", msg)
    }
}
