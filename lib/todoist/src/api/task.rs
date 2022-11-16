use crate::models;
crate::endpoint_group!();

impl Client<'_> {
    pub async fn list_active(&self) -> Vec<models::Task> {
        match self.api_client.get("/tasks".to_string()).await {
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

