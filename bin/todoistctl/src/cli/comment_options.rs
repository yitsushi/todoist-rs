use clap::Args;
use libtodoist::api::comment::{ListRequest, CreateRequest, UpdateRequest};

#[derive(Args,Debug,Clone)]
pub struct ListOptions {
    #[clap(long)]
    pub project_id: Option<String>,
    #[clap(long)]
    pub task_id: Option<String>,
}

impl From<ListOptions> for ListRequest {
    fn from(opts: ListOptions) -> Self {
        ListRequest {
            project_id: opts.project_id,
            task_id: opts.task_id,
        }
    }
}

#[derive(Args,Debug,Clone)]
pub struct ShowOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct DeleteOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct NewOptions {
    #[clap(long)]
    pub content: String,
    #[clap(long)]
    pub task_id: Option<String>,
    #[clap(long)]
    pub project_id: Option<String>,
}

impl From<NewOptions> for CreateRequest {
    fn from(opts: NewOptions) -> Self {
        CreateRequest {
            content: opts.content,
            project_id: opts.project_id,
            task_id: opts.task_id,
            attachment: None,
        }
    }
}

#[derive(Args,Debug,Clone)]
pub struct UpdateOptions {
    #[clap(long)]
    pub id: String,
    #[clap(long)]
    pub content: String,
}

impl From<UpdateOptions> for UpdateRequest {
    fn from(opts: UpdateOptions) -> Self {
        UpdateRequest {
            content: opts.content,
        }
    }
}
