use core::option::Option;
use clap::{Args, Subcommand};
use todoist::api::project::{CreateRequest, UpdateRequest};
use todoist::enums::ViewStyle;

use super::project_options::*;

#[derive(Args,Debug)]
pub struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug,Clone)]
pub enum Action {
    List,
    Delete(DeleteOptions),
    New(NewOptions),
    Update(UpdateOptions),
    Show(ShowOptions),
    Collaborators(CollaboratorsOptions),
}

impl Cli {
    pub async fn run(&self, client: &todoist::Client) {
        match self.action.clone() {
            Action::List => {
                for project in client.project().list().await {
                    if let Some(_) = project.parent_id {
                        println!("  - {}", project);
                    } else {
                        println!("{}", project);
                    }
                }
            }
            Action::Delete(opts) => {
                client.project().delete(opts.id).await;
            }
            Action::New(opts) => {
                let _ = client.project().create(CreateRequest {
                    name: opts.name,
                    parent_id: opts.parent_id,
                    color: opts.color,
                    is_favorite: opts.is_favorite,
                    view_style: opts.view_style.unwrap_or(ViewStyle::List),
                }).await;
            }
            Action::Update(opts) => {
                let _ = client.project().update(opts.id, UpdateRequest {
                    name: opts.name,
                    color: opts.color,
                    is_favorite: opts.is_favorite,
                    view_style: opts.view_style,
                }).await;
            }
            Action::Show(opts) => {
                match client.project().get(opts.id).await {
                    Ok(Some(proj)) => { println!("{:#?}", proj); },
                    Ok(None) => println!("project not found"),
                    Err(err) => println!("{}", err),
                }
            }
            Action::Collaborators(opts) => {
                match client.project().collaborators(opts.id).await {
                    Err(err) => println!("{}", err),
                    Ok(collaborators) => {
                        println!("Collaborators:");
                        for collaborator in collaborators {
                            println!(" - {}", collaborator)
                        }
                    },
                }
            }
        }
    }
}
