use serde::Serialize;

use crate::enums::{Color, ViewStyle};
use crate::error::Error;
use crate::models;

crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self) -> Vec<models::Project> {
        match self.api_client.get("/projects".to_string()).await {
            Err(err) => { println!("{}", err); vec![] },
            Ok(None) => { println!("no content"); vec![] }
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(projects) => projects,
                    Err(err) => { println!("{}", err); vec![] },
                }
            },
        }
    }

    pub async fn create(&self, request: CreateProjectRequest) -> Result<Option<models::Project>, Error> {
        match self.api_client.post("/projects".to_string(), request).await {
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

    pub async fn delete(&self, id: String) -> Option<Error> {
        match self.api_client.delete(format!("/projects/{}", id)).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => { println!("no content"); None },
        }
    }
}

#[derive(Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub parent_id: Option<String>,
    pub color: Option<Color>,
    pub is_favorite: bool,
    pub view_style: ViewStyle,
}