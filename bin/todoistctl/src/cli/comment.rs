use core::option::Option;
use clap::{Args, Subcommand};

use super::comment_options::*;

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
                match client.comment().list(opts.into()).await {
                    Err(err) => println!(" -- [ERROR] {}", err),
                    Ok(comments) =>
                        for comment in comments {
                            println!("{}", comment);
                        }
                }
            }
            Action::New(opts) => {
                match client.comment().create(opts.into()).await {
                    Ok(Some(comment)) => { println!("{:#?}", comment); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            }
            Action::Show(opts) => {
                match client.comment().get(opts.id).await {
                    Ok(Some(comment)) => { println!("{:#?}", comment); },
                    Ok(None) => println!("task not found"),
                    Err(err) => println!("{}", err),
                }
            },
            Action::Update(opts) => {
                match client.comment().update(opts.id.clone(), opts.into()).await {
                    Ok(Some(comment)) => { println!("{:#?}", comment); },
                    Ok(None) => { println!("something went wrong"); },
                    Err(err) => { println!("error: {}", err); },
                }
            },
            Action::Delete(opts) => {
                client.comment().delete(opts.id).await;
            },
        }
    }
}
