mod project;

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
    client: todoist::Client,
}

impl Cli {
    pub fn new(client: todoist::Client) -> Self {
        let mut cli: Cli = Cli::parse();

        cli.set_client(client);

        return cli
    }

    fn set_client(&mut self, client: todoist::Client) {
        self.client = client
    }

    pub async fn run(&self) {
        match &self.action {
            GroupAction::Project(project) => project.run(&self.client).await,
            GroupAction::Task => {}
            GroupAction::Section => {}
            GroupAction::Comment => {}
            GroupAction::Label => {}
        }
    }
}

#[derive(Subcommand,Debug)]
pub enum GroupAction {
    Project(project::Cli),
    Task,
    Section,
    Comment,
    Label,
}