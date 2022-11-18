use serde::Serialize;

use crate::enums::Priority;
use crate::error::Error;
use crate::{models, EmptyQuery};

crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self, params: ListRequest) -> Vec<models::Task> {
        match self.api_client.get("/tasks".to_string(), &params).await {
            Err(err) => { println!("{}", err); vec![] },
            Ok(None) => { println!("no content"); vec![] },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(projects) => projects,
                    Err(err) => { println!("{}", err); vec![] },
                }
            },
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<models::Task>, Error> {
        match self.api_client.get(format!("/tasks/{}", id), &EmptyQuery{}).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(task) => Ok(Some(task)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }

    pub async fn create(&self, request: CreateRequest) -> Result<Option<models::Task>, Error> {
        match self.api_client.post("/tasks".to_string(), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(task) => Ok(Some(task)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }

    pub async fn update(&self, id: String, request: UpdateRequest) -> Result<Option<models::Task>, Error> {
        match self.api_client.post(format!("/tasks/{}", id), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(task) => Ok(Some(task)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }

    pub async fn close(&self, id: String) -> Option<Error> {
        match self.api_client.post(format!("/tasks/{}/close", id), EmptyQuery{}).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => None,
        }
    }

    pub async fn reopen(&self, id: String) -> Option<Error> {
        match self.api_client.post(format!("/tasks/{}/reopen", id), EmptyQuery{}).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => None,
        }
    }

    pub async fn delete(&self, id: String) -> Option<Error> {
        match self.api_client.delete(format!("/tasks/{}", id)).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => None,
        }
    }
}

#[derive(Serialize)]
pub struct ListRequest {
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub label: Option<String>,
    pub filter: Option<String>,
    pub lang: Option<String>,
    pub ids: Option<String>,
}

#[derive(Serialize)]
pub struct CreateRequest {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<i64>,
    pub labels: Vec<String>,
    pub priority: Option<Priority>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub assignee_id: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateRequest {
    pub content: Option<String>,
    pub description: Option<String>,
    pub labels: Vec<String>,
    pub priority: Option<Priority>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub assignee_id: Option<String>,
}
