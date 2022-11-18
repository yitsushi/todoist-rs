mod cli;

use std::collections::HashMap;
use std::{thread, time};

use todoist::enums::{Color, ViewStyle};
use todoist::models;

const TOKEN_ENV_NAME: &str = "TODOIST_TOKEN";


#[tokio::main]
async fn main() {
    let token = match std::env::var(TOKEN_ENV_NAME) {
        Ok(val) => val,
        Err(_) => {
            println!("TODOIST_TOKEN is not defined");
            return
        },
    };

    let client = todoist::Client::new(token);

    let app: cli::Cli = cli::Cli::new(client);

    app.run().await;
}

#[allow(dead_code)]
async fn checks(token: String) {
    let client = todoist::Client::new(token);

    let mut projects: HashMap<String, models::Project> = HashMap::new();

    for project in client.project().list().await {
        projects.insert(project.id.clone(), project.clone());
    }

    // for task in client.task().list().await {
    //     match projects.get(&task.project_id.unwrap()) {
    //         Some(project) => println!("<{} :: {}> {}", project.name, project.id, task.content),
    //         None => {},
    //     }
    // }

    let resp = client.project().create(todoist::api::project::CreateRequest {
        name: "test project".to_string(),
        parent_id: None,
        color: Some(Color::Lavender),
        is_favorite: false,
        view_style: ViewStyle::List
    }).await;

    let to_delete = match resp {
        Ok(Some(proj)) => proj.id,
        _ => return,
    };

    let ten_secs = time::Duration::from_secs(5);

    thread::sleep(ten_secs);

    match client.project().update(to_delete.clone(), todoist::api::project::UpdateRequest{
        name: Some("new name for test".to_string()),
        color: Some(Color::Yellow),
        is_favorite: None,
        view_style: None
    }).await {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    }

    match client.project().get(to_delete.clone()).await {
        Ok(Some(proj2)) => println!("{:?}", proj2),
        _ => println!("nope, didn't work")
    }

    thread::sleep(ten_secs);

    if let Some(resp) = client.project().delete(to_delete.to_string()).await {
        println!("{}", resp);
    }
}