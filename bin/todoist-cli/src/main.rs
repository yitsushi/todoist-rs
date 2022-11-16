use std::collections::HashMap;
use std::{thread, time};
use todoist::enums::{Color, ViewStyle};

const TOKEN_ENV_NAME: &str = "TODOIST_TOKEN";

#[tokio::main]
async fn main() {
    let token = match std::env::var(TOKEN_ENV_NAME) {
        Ok(val) => val,
        Err(_) => { println!("TODOIST_TOKEN is not defined"); return },
    };

    println!("token: {}", token);

    let client = todoist::Client::new(token);

    let mut projects: HashMap<String, String> = HashMap::new();

    for project in client.project().list().await {
        projects.insert(project.id, project.name);
    }

    for task in client.task().list_active().await {
        let project = match projects.get(&task.project_id.unwrap()) {
            Some(project_name) => project_name,
            None => "no project",
        };

        println!("<{}> {}", project, task.content);
    }

    let resp = client.project().create(todoist::api::project::CreateProjectRequest{
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

    let ten_secs = time::Duration::from_secs(10);
    thread::sleep(ten_secs);

    if let Some(resp) = client.project().delete(to_delete.to_string()).await {
        println!("{}", resp);
    }
}
