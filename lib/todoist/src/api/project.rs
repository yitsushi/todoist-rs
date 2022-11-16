use crate::models;
crate::endpoint_group!();

impl Client<'_> {
    pub async fn list(&self) -> Vec<models::Project> {
        match self.api_client.get("/projects".to_string()).await {
            Ok(Some(text)) => {
                match serde_json::from_str(&text) {
                    Ok(projects) => projects,
                    Err(err) => { println!("{}", err); vec![] },
                }
            },
            Ok(None) => { println!("no content"); vec![] }
            Err(err) => { println!("{}", err); vec![] }
        }
    }
}