use clap::Args;
use todoist::{api::task::{CreateRequest, ListRequest}, enums::Priority};

#[derive(Args,Debug,Clone)]
pub struct ListOptions {
    #[clap(long)]
    pub project_id: Option<String>,
    #[clap(long)]
    pub section_id: Option<String>,
    #[clap(long)]
    pub label: Option<String>,
    #[clap(long)]
    pub filter: Option<String>,
    #[clap(long)]
    pub lang: Option<String>,
    #[clap(long)]
    pub ids: Option<String>,
}

#[derive(Args,Debug,Clone)]
pub struct NewOptions {
    #[clap(long)]
    pub content: String,
    #[clap(long)]
    pub description: Option<String>,
    #[clap(long)]
    pub project_id: Option<String>,
    #[clap(long)]
    pub section_id: Option<String>,
    #[clap(long)]
    pub parent_id: Option<String>,
    #[clap(long)]
    pub order: Option<i64>,
    #[clap(long)]
    pub labels: Vec<String>,
    #[clap(long)]
    pub priority: Option<Priority>,
    #[clap(long)]
    pub due_string: Option<String>,
    #[clap(long)]
    pub due_date: Option<String>,
    #[clap(long)]
    pub due_datetime: Option<String>,
    #[clap(long)]
    pub due_lang: Option<String>,
    #[clap(long)]
    pub assignee_id: Option<String>,
}

#[derive(Args,Debug,Clone)]
pub struct ShowOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct CloseOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct ReopenOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct DeleteOptions {
    #[clap(long)]
    pub id: String,
}

impl From<ListOptions> for ListRequest {
    fn from(opts: ListOptions) -> Self {
        ListRequest{
            project_id: opts.project_id,
            section_id: opts.section_id,
            label: opts.label,
            filter: opts.filter,
            lang: opts.lang,
            ids: opts.ids,
        }
    }
}

impl From<NewOptions> for CreateRequest {
    fn from(opts: NewOptions) -> Self {
        CreateRequest{
            content: opts.content,
            description: opts.description,
            project_id: opts.project_id,
            section_id: opts.section_id,
            parent_id: opts.parent_id,
            order: opts.order,
            labels: opts.labels,
            priority: opts.priority,
            due_string: opts.due_string,
            due_date: opts.due_date,
            due_datetime: opts.due_datetime,
            due_lang: opts.due_lang,
            assignee_id: opts.assignee_id,
        }
    }
}
