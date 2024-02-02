use serde::Serialize;

use crate::error::Error;
use crate::models::Attachment;
use crate::{models, EmptyQuery};

crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self, params: ListRequest) -> Result<Vec<models::Comment>, Error> {
        match self.api_client.get("/comments".to_string(), &params).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(vec![]),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(comments) => Ok(comments),
                Err(err) => Err(Error::ParseError(err.to_string())),
            },
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<models::Comment>, Error> {
        match self.api_client.get(format!("/comments/{}", id), &EmptyQuery{}).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(comment) => Ok(Some(comment)),
                Err(err) => Err(Error::ParseError(err.to_string()))
            },
        }
    }

    pub async fn create(&self, request: CreateRequest) -> Result<Option<models::Comment>, Error> {
        match self.api_client.post("/comments".to_string(), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(comment) => Ok(Some(comment)),
                Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
            },
        }
    }

    pub async fn update(&self, id: String, request: UpdateRequest) -> Result<Option<models::Comment>, Error> {
        match self.api_client.post(format!("/comments/{}", id), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(comment) => Ok(Some(comment)),
                Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
            },
        }
    }

    pub async fn delete(&self, id: String) -> Option<Error> {
        match self.api_client.delete(format!("/comments/{}", id)).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => None,
        }
    }
}

#[derive(Serialize)]
pub struct ListRequest {
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Serialize)]
pub struct CreateRequest {
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub content: String,
    pub attachment: Option<Attachment>,
}

#[derive(Serialize)]
pub struct UpdateRequest {
    pub content: String,
}
