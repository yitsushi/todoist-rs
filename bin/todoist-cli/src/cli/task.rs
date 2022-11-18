use core::convert::From;
use core::option::Option;
use clap::{Args, Subcommand};
use todoist::api::task::{CreateRequest, ListRequest};

#[derive(Args,Debug)]
pub struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug,Clone)]
pub enum Action {
    List(ListOptions),
    New(NewOptions),
    Show,
    Update,
    Close,
    Reopen,
    Delete,
}

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
    content: String,
    #[clap(long)]
    description: Option<String>,
    #[clap(long)]
    project_id: Option<String>,
    #[clap(long)]
    section_id: Option<String>,
    #[clap(long)]
    parent_id: Option<String>,
    #[clap(long)]
    order: Option<i64>,
    #[clap(long)]
    labels: Vec<String>,
    #[clap(long)]
    priority: Option<u8>,
    #[clap(long)]
    due_string: Option<String>,
    #[clap(long)]
    due_date: Option<String>,
    #[clap(long)]
    due_datetime: Option<String>,
    #[clap(long)]
    due_lang: Option<String>,
    #[clap(long)]
    assignee_id: Option<String>,
}

impl Cli {
    pub async fn run(&self, client: &todoist::Client) {
        match self.action.clone() {
            Action::List(opts) => {
                for task in client.task().list(opts.into()).await {
                    println!("[{}] {}", task.project_id.unwrap(), task.content);
                }
            }
            Action::New(opts) => {
                match client.task().create(opts.into()).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            }
            Action::Show => todo!(),
            Action::Update => todo!(),
            Action::Close => todo!(),
            Action::Reopen => todo!(),
            Action::Delete => todo!(),
        }
    }
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
