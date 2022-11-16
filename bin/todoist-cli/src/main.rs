use std::collections::HashMap;

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
}
