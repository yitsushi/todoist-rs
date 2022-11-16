pub mod api;
pub mod models;
pub mod error;

use reqwest::RequestBuilder;
use serde::Serialize;
use error::Error;
use api::{project, task};

macro_rules! endpoint_fn {
    ($name:ident) => {
        pub fn $name(&self) -> $name::Client {
            $name::new(self)
        }
    };
}

pub struct Client {
    api_token: String,
}

impl Client {
    pub fn new(api_token: String) -> Self {
        Self{ api_token }
    }

    fn token(&self) -> String {
        return self.api_token.clone()
    }

    fn v2(&self, path: String) -> String {
        format!("https://api.todoist.com/rest/v2{}", path)
    }

    async fn send(&self, request: RequestBuilder) -> Result<Option<String>, Error> {
        let result = request
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.token()))
            .send().await;

        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.text().await {
                    Ok(text) => Ok(Some(text)),
                    Err(err) => Err(Error::ParseError(err.to_string())),
                },
                reqwest::StatusCode::NO_CONTENT => {Ok(None)},
                c => {
                    let msg = match c.as_u16() / 100 {
                        4 => "request was invalid and should not be retried unmodified",
                        5 => "server error, it's safe to retry later",
                        _ => "invalid status code",
                    };
                    Err(Error::ServerError(c, msg.to_string()))
                }
            },
            Err(err) => Err(Error::RequestError(err.to_string())),
        }
    }

    pub async fn get(&self, path: String) -> Result<Option<String>, Error> {
        let client = reqwest::Client::new();
        let request = client.get(self.v2(path));

        self.send(request).await
    }

    pub async fn post(&self, path: String, data: impl Serialize) -> Result<Option<String>, Error> {
        let client = reqwest::Client::new();
        let request = client.post(self.v2(path))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&data).unwrap());

        self.send(request).await
    }

    endpoint_fn!(project);
    endpoint_fn!(task);
}