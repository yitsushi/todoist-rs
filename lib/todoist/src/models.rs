use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub comment_count: i64,
    pub parent_id: Option<String>,
    pub order: i64,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub is_team_inbox: bool,
    pub view_style: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub creator_id: String,
    pub assignee: Option<String>,
    pub assigner: Option<String>,
    pub created_at: String,
    pub comment_count: i64,
    pub is_completed: bool,
    pub content: String,
    pub description: String,
    pub due: Option<Due>,
    pub id: String,
    pub labels: Vec<String>,
    pub order: i64,
    pub priority: u8,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Due {
    pub date: Option<String>,
    pub datetime: Option<String>,
    pub is_recurring: bool,
    pub string: String,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub email: String,
}
