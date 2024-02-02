use core::convert::From;
use core::option::Option;
use clap::{Args, Subcommand};

use super::section_options::*;

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
    Delete(DeleteOptions),
}

impl Cli {
    pub async fn run(&self, client: &libtodoist::Client) {
        match self.action.clone() {
            Action::List(opts) => {
                match client.section().list(opts.project_id).await {
                    Err(err) => println!(" -- [ERROR] {}", err),
                    Ok(sections) => {
                        for section in sections {
                            println!("{}", section);
                        }
                    },
                }
            }
            Action::New(opts) => {
                match client.section().create(opts.into()).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            }
            Action::Show(opts) => {
                match client.section().get(opts.id).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => println!("task not found"),
                    Err(err) => println!("{}", err),
                }
            },
            Action::Update(opts) => {
                match client.section().update(opts.id.clone(), opts.into()).await {
                    Ok(Some(task)) => { println!("{:#?}", task); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            },
            Action::Delete(opts) => {
                client.section().delete(opts.id).await;
            },
        }
    }
}

