use clap::Args;
use todoist::enums::{Color, ViewStyle};

#[derive(Args,Debug,Clone)]
pub struct NewOptions {
    #[clap(long)]
    pub name: String,
    #[clap(long)]
    pub parent_id: Option<String>,
    #[clap(long)]
    pub color: Option<Color>,
    #[clap(long, default_value_t=false)]
    pub is_favorite: bool,
    #[clap(long)]
    pub view_style: Option<ViewStyle>,
}

#[derive(Args,Debug,Clone)]
pub struct UpdateOptions {
    #[clap(long)]
    pub id: String,
    #[clap(long)]
    pub name: Option<String>,
    #[clap(long)]
    pub color: Option<Color>,
    #[clap(long)]
    pub is_favorite: Option<bool>,
    #[clap(long)]
    pub view_style: Option<ViewStyle>,
}

#[derive(Args,Debug,Clone)]
pub struct DeleteOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct ShowOptions {
    #[clap(long)]
    pub id: String,
}

#[derive(Args,Debug,Clone)]
pub struct CollaboratorsOptions {
    #[clap(long)]
    pub id: String,
}
