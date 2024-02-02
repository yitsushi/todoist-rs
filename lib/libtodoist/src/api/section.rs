use serde::Serialize;

use crate::error::Error;
use crate::{EmptyQuery, models};

crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self, project_id: Option<String>) -> Result<Vec<models::Section>, Error> {
        match self.api_client.get("/sections".to_string(), &ListRequest{ project_id }).await {
            Err(err) => Err(err),
            Ok(None) => Ok(vec![]),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(sections) => Ok(sections),
                Err(err) => Err(Error::RequestError(err.to_string())),
            },
        }
    }

    pub async fn create(&self, request: CreateRequest) -> Result<Option<models::Section>, Error> {
        match self.api_client.post("/sections".to_string(), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(section) => Ok(Some(section)),
                Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
            },
        }
    }

    pub async fn delete(&self, id: String) -> Option<Error> {
        match self.api_client.delete(format!("/sections/{}", id)).await {
            Err(err) => Some(Error::RequestError(err.to_string())),
            Ok(_) => None,
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<models::Section>, Error> {
        match self.api_client.get(format!("/sections/{}", id), &EmptyQuery{}).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(section) => Ok(Some(section)),
                Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
            },
        }
    }

    pub async fn update(&self, id: String, request: UpdateRequest) -> Result<Option<models::Section>, Error> {
        match self.api_client.post(format!("/sections/{}", id), request).await {
            Err(err) => Err(Error::RequestError(err.to_string())),
            Ok(None) => Ok(None),
            Ok(Some(text)) => match serde_json::from_str(&text) {
                Ok(section) => Ok(Some(section)),
                Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
            },
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

#[cfg(test)]
mod tests {
    use mockito::{self, Mock};
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "fixtures/"]
    struct Fixtures;

    fn load_fixture(path: &str) -> Result<String, String> {
        match Fixtures::get(path) {
            Some(file) => Ok(String::from_utf8_lossy(file.data.clone().as_ref()).to_string()),
            None => Err(format!("fixture file not found: {}", path)),
        }
    }

    fn new_mock(method: &str, path: &str, status_code: usize, response: &str) -> (crate::Client, Mock) {
        let fullpath = format!("/rest/v2{}", path);
        let mut server = mockito::Server::new();
        let host = server.host_with_port();
        let mock = server.mock(method, fullpath.as_str())
            .match_header("authorization", "Bearer token")
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(response);

        let client = crate::Client::new_with_base_url(String::from("token"), format!("http://{}/rest/v2", host));

        (client, mock)
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>>  {
        let (client, mock) = new_mock("GET", "/sections?project_id=2203306141", 200, &load_fixture("sections/list.json").unwrap());
        let mock = mock.create();

        let response = client.section().list(Some("2203306141".to_string())).await;

        mock.assert();

        assert_eq!(response.unwrap().len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>>  {
        let (client, mock) = new_mock("GET", "/sections/7026", 200, &load_fixture("sections/get.json").unwrap());
        let mock = mock.create();

        let response = client.section().get("7026".to_string()).await;

        mock.assert();

        let section = response.unwrap().unwrap();

        assert_eq!(section.id, "7026");
        assert_eq!(section.name, "Definitely Not Groceries");
        assert_eq!(section.project_id, "2203306141");
        assert_eq!(section.order, 2);

        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>>  {
        let (client, mock) = new_mock("DELETE", "/sections/7026", 204, "");
        let mock = mock.create();

        let response = client.section().delete("7026".to_string()).await;

        mock.assert();

        assert!(response.is_none());

        Ok(())
    }
}
