use std::fmt::Display;

use serde::{Serialize, Deserialize};
use super::enums::*;

const ICON_FAVORITE: &str = "★";
const ICON_SHARED: &str = "⛹";
const ICON_INBOX: &str = "✉ ";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub comment_count: i64,
    pub parent_id: Option<String>,
    pub order: i64,
    pub color: Color,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub is_team_inbox: bool,
    pub view_style: ViewStyle,
    pub url: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fav_icon = if self.is_favorite { ICON_FAVORITE } else { "" };
        let shared_icon = if self.is_shared { ICON_SHARED } else { "" };
        let inbox_icon = if self.is_inbox_project { ICON_INBOX } else { "" };

        write!(f, "{} [{}] {}{}{}", self.name, self.id, shared_icon, inbox_icon, fav_icon)
    }
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
    pub priority: Priority,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub url: String,
    pub duraction: Option<Duration>,
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
pub struct Duration {
    pub amount: i64,
    pub unit: DurationUnit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Display for Collaborator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub posted_at: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub attachment: Option<Attachment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub file_name: String,
    pub file_type: String,
    pub file_url: String,
    pub resource_type: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = &self.content;
        let posted_at = &self.posted_at;
        let id = &self.id;
        let parent = if let Some(task_id) = &self.task_id {
            Some(task_id)
        } else if let Some(project_id) = &self.project_id {
            Some(project_id)
        } else {
            None
        };

        if let Some(parent_id) = parent {
            write!(f, "ID: {}\nParent: {}\n{}\n<{}>", id, parent_id, content, posted_at)
        } else {
            write!(f, "ID: {}\n{}\n<{}>", id, content, posted_at)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub order: i64,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] <project:{}> {}", self.id, self.project_id, self.name)
    }
}
