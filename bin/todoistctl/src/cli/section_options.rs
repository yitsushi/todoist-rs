use clap::Args;
use libtodoist::api::section::{CreateRequest, UpdateRequest};

#[derive(Args,Debug,Clone)]
pub struct ListOptions {
    #[clap(long)]
    pub project_id: Option<String>,
}

#[derive(Args,Debug,Clone)]
pub struct NewOptions {
    #[clap(long)]
    pub name: String,
    #[clap(long)]
    pub project_id: String,
    #[clap(long)]
    pub order: Option<i64>,
}

#[derive(Args,Debug,Clone)]
pub struct UpdateOptions {
    #[clap(long)]
    pub id: String,
    #[clap(long)]
    pub name: String,
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

impl From<UpdateOptions> for UpdateRequest {
    fn from(opts: UpdateOptions) -> Self {
        UpdateRequest{
            name: opts.name,
        }
    }
}

impl From<NewOptions> for CreateRequest {
    fn from(opts: NewOptions) -> Self {
        CreateRequest{
            name: opts.name,
            project_id: opts.project_id,
            order: opts.order,
        }
    }
}
