pub mod api;
pub mod models;
pub mod error;
pub mod enums;

use core::option::Option;
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

#[derive(Serialize)]
pub struct EmptyQuery {}

#[derive(Default)]
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

    pub async fn get<T: Serialize>(&self, path: String, query: &T) -> Result<Option<String>, Error> {
        let client = reqwest::Client::new();
        let request = client.get(self.v2(path)).query(query);

        self.send(request).await
    }

    pub async fn post<T: Serialize>(&self, path: String, data: T) -> Result<Option<String>, Error> {
        let client = reqwest::Client::new();
        let request = client.post(self.v2(path))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&data).unwrap());

        self.send(request).await
    }

    pub async fn delete(&self, path: String) -> Result<Option<String>, Error> {
        let client = reqwest::Client::new();
        let request = client.delete(self.v2(path))
            .header(reqwest::header::CONTENT_TYPE, "application/json");

        self.send(request).await
    }

    endpoint_fn!(project);
    endpoint_fn!(task);
}