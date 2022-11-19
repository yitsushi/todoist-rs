use serde::Serialize;

use crate::enums::{Color, ViewStyle};
use crate::error::Error;
use crate::{EmptyQuery, models};

crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self, project_id: Option<String>) -> Vec<models::Section> {
        match self.api_client.get("/sections".to_string(), &ListRequest{ project_id }).await {
            Err(err) => { println!("{}", err); vec![] },
            Ok(None) => { println!("no content"); vec![] }
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(sections) => sections,
                    Err(err) => { println!("{}", err); vec![] },
                }
            },
        }
    }

    pub async fn create(&self, request: CreateRequest) -> Result<Option<models::Section>, Error> {
        match self.api_client.post("/sections".to_string(), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(section) => Ok(Some(section)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }

    pub async fn delete(&self, id: String) -> Option<Error> {
        match self.api_client.delete(format!("/sections/{}", id)).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => { None },
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<models::Section>, Error> {
        match self.api_client.get(format!("/sections/{}", id), &EmptyQuery{}).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(proj) => Ok(Some(proj)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }

    pub async fn update(&self, id: String, request: UpdateRequest) -> Result<Option<models::Section>, Error> {
        match self.api_client.post(format!("/sections/{}", id), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => { println!("no content"); Ok(None) },
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(proj) => Ok(Some(proj)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct ListRequest {
    pub project_id: Option<String>,
}

#[derive(Serialize)]
pub struct CreateRequest {
    pub name: String,
    pub project_id: String,
    pub order: Option<i64>,
}

#[derive(Serialize)]
pub struct UpdateRequest {
    pub name: String,
}