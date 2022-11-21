mod comment;
mod comment_options;
mod project;
mod project_options;
mod section;
mod section_options;
mod task;
mod task_options;

use core::option::Option;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author="Efertone <efertone@pm.me>", version, about="Todoist CLI")]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(short, long)]
    verbose: bool,
    #[command(subcommand)]
    action: GroupAction,

    #[clap(skip)]
    client: libtodoist::Client,
}

impl Cli {
    pub fn new(client: libtodoist::Client) -> Self {
        let mut cli: Cli = Cli::parse();

        cli.set_client(client);

        return cli
    }

    fn set_client(&mut self, client: libtodoist::Client) {
        self.client = client
    }

    pub async fn run(&self) {
        match &self.action {
            GroupAction::Project(project) => project.run(&self.client).await,
            GroupAction::Task(task) => task.run(&self.client).await,
            GroupAction::Section(section) => section.run(&self.client).await,
            GroupAction::Comment(comment) => comment.run(&self.client).await,
            GroupAction::Label => {}
        }
    }
}

#[derive(Subcommand,Debug)]
pub enum GroupAction {
    Project(project::Cli),
    Task(task::Cli),
    Section(section::Cli),
    Comment(comment::Cli),
    Label,
}
