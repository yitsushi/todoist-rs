mod cli;

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    token: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { token: "".into() }
    }
}

#[tokio::main]
async fn main() {
    let cfg: Config = match confy::load("todoist-cli", "config") {
        Ok(c) => c,
        Err(err) => { println!("{}", err) ; return },
    };

    let client = todoist::Client::new(cfg.token);

    let app: cli::Cli = cli::Cli::new(client);

    app.run().await;
}
