use core::option::Option;
use clap::{Args, Parser, Subcommand};

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
            GroupAction::Project(project) => self.project(project).await,
            GroupAction::Task => {}
            GroupAction::Section => {}
            GroupAction::Comment => {}
            GroupAction::Label => {}
        }
    }

    pub async fn project(&self, args: &Project) {
        match &args.action {
            ProjectAction::List => {
                for project in self.client.project().list().await {
                    println!("[{}] {}", project.id, project.name);
                }
            }
            ProjectAction::Delete => {}
            ProjectAction::New(opts) => {
                println!("{}", opts.name)
            }
            ProjectAction::Show => {}
        }
    }
}

#[derive(Subcommand,Debug)]
pub enum GroupAction {
    Project(Project),
    Task,
    Section,
    Comment,
    Label,
}

#[derive(Args,Debug)]
pub struct Project {
    #[command(subcommand)]
    action: ProjectAction,
}

#[derive(Subcommand,Debug)]
pub enum ProjectAction {
    List,
    Delete,
    New(ProjectNewOptions),
    Show,
}

#[derive(Args,Debug)]
pub struct ProjectNewOptions {
    #[clap(long)]
    name: String,
    #[clap(long)]
    parent_id: Option<String>,
    #[clap(long)]
    view_style: Option<String>,
}