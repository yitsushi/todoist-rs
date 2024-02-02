pub mod personal {
    use serde::Serialize;

    use crate::enums::Color;
    use crate::{models, EmptyQuery};
    use crate::error::Error;

    crate::endpoint_group!();

    impl Client<'_> {
        pub async fn list(&self) -> Result<Vec<models::Label>, Error> {
            match self.api_client.get("/labels".to_string(), &EmptyQuery{}).await {
                Err(err) => Err(Error::RequestError(err.to_string())),
                Ok(None) => Ok(vec![]),
                Ok(Some(text)) => match serde_json::from_str(&text) {
                    Ok(labels) => Ok(labels),
                    Err(err) => Err(Error::ParseError(err.to_string())),
                },
            }
        }

        pub async fn create(&self, request: CreateRequest) -> Result<Option<models::Label>, Error> {
            match self.api_client.post("/labels".to_string(), request).await {
                Err(err) => Err(Error::RequestError(err.to_string())),
                Ok(None) => Ok(None),
                Ok(Some(text)) => match serde_json::from_str(&text) {
                    Ok(label) => Ok(Some(label)),
                    Err(err) => Err(Error::ParseError(err.to_string())),
                },
            }
        }

        pub async fn get(&self, id: String) -> Result<Option<models::Label>, Error> {
            match self.api_client.get(format!("/labels/{}", id), &EmptyQuery{}).await {
                Err(err) => Err(Error::RequestError(err.to_string())),
                Ok(None) => Ok(None),
                Ok(Some(text)) => match serde_json::from_str(&text) {
                    Ok(label) => Ok(Some(label)),
                    Err(err) => Err(Error::ParseError(err.to_string()))
                },
            }
        }

        pub async fn delete(&self, id: String) -> Option<Error> {
            match self.api_client.delete(format!("/labels/{}", id)).await {
                Err(err) => Some(Error::RequestError(err.to_string())),
                Ok(_) => None,
            }
        }

        pub async fn update(&self, id: String, request: UpdateRequest) -> Result<Option<models::Label>, Error> {
            match self.api_client.post(format!("/labels/{}", id), request).await {
                Err(err) => Err(Error::RequestError(err.to_string())),
                Ok(None) => Ok(None),
                Ok(Some(text)) => match serde_json::from_str(&text) {
                    Ok(label) => Ok(Some(label)),
                    Err(_) => Err(Error::ParseError("unable to parse response".to_string()))
                },
            }
        }
    }

    #[derive(Serialize)]
    pub struct CreateRequest {
        pub name: String,
        pub order: i64,
        pub color: Color,
        pub is_favorite: bool,
    }

    #[derive(Serialize)]
    pub struct UpdateRequest {
        pub name: String,
        pub order: i64,
        pub color: Color,
        pub is_favorite: bool,
    }
}

pub mod shared {
    use serde::Serialize;

    use crate::{models, error::Error};

    crate::endpoint_group!();

    impl Client<'_> {
        pub async fn list(&self, omit_personal: bool) -> Result<Vec<models::Label>, Error> {
            match self.api_client.get("/labels".to_string(), &ListRequest{omit_personal}).await {
                Err(err) => Err(Error::RequestError(err.to_string())),
                Ok(None) => Ok(vec![]),
                Ok(Some(text)) => match serde_json::from_str(&text) {
                    Ok(labels) => Ok(labels),
                    Err(err) => Err(Error::ParseError(err.to_string())),
                },
            }
        }

        pub async fn rename(&self, request: RenameRequest) -> Option<Error> {
            match self.api_client.post("/labels/shared/rename".to_string(), request).await {
                Err(err) => Some(Error::RequestError(err.to_string())),
                Ok(_) => None,
            }
        }

        pub async fn remove(&self, name: String) -> Option<Error> {
            match self.api_client.post("/labels/shared/rename".to_string(), &RemoveRequest{name}).await {
                Err(err) => Some(Error::RequestError(err.to_string())),
                Ok(_) => None,
            }
        }
    }

    #[derive(Serialize)]
    pub struct ListRequest {
        pub omit_personal: bool,
    }

    #[derive(Serialize)]
    pub struct RenameRequest {
        pub name: String,
        pub new_name: String,
    }

    #[derive(Serialize)]
    pub struct RemoveRequest {
        pub name: String,
    }
}
