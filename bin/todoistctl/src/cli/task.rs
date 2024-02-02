use core::convert::From;
use core::option::Option;
use clap::{Args, Subcommand};

use super::task_options::*;

#[derive(Args,Debug)]
pub struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug,Clone)]
pub enum Action {
    List(ListOptions),
    New(NewOptions),
    Show(ShowOptions),
    Update(UpdateOptions),
    Close(CloseOptions),
    Reopen(ReopenOptions),
    Delete(DeleteOptions),
}

impl Cli {
    pub async fn run(&self, client: &libtodoist::Client) {
        match self.action.clone() {
            Action::List(opts) => {
                match client.task().list(opts.clone().into()).await {
                    Err(err) => println!(" -- [ERROR] {}", err),
                    Ok(tasks) => 
                        for task in tasks {
                            let due = if let Some(value) = &task.due {
                                format!("{}", value)
                            } else {
                                String::from("none")
                            };

                            if opts.with_description {
                                println!("[project:{}] <{}> (due: {}) {} -> {}", task.project_id.unwrap(), task.id, due, task.content, task.description);
                            } else {
                                println!("[project:{}] <{}> (due: {}) {}", task.project_id.unwrap(), task.id, due, task.content);
                            }
                        }
                }
            }
            Action::New(opts) => {
                match client.task().create(opts.into()).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            }
            Action::Show(opts) => {
                match client.task().get(opts.id).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => println!("task not found"),
                    Err(err) => println!("{}", err),
                }
            },
            Action::Update(opts) => {
                match client.task().update(opts.id.clone(), opts.into()).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            },
            Action::Close(opts) => {
                client.task().close(opts.id).await;
            },
            Action::Reopen(opts) => {
                client.task().reopen(opts.id).await;
            },
            Action::Delete(opts) => {
                client.task().delete(opts.id).await;
            },
        }
    }
}

