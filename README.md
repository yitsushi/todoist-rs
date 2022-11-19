# Todoist crate and cli

Heavily WIP, the CLI is *NOT* user friendly at all, hopefully it will be, but
not yet. The library can be used, endpoints will be implemented as I go through
the endpoint list, I'll add them to the library and the CLI at the same time.

Disclaimer: This is my first Rust project, if you are a Rust veteran, feel free
to make suggestions as issues for example, I'm more than happy to learn, but
I'd rather not learn from reading huge PRs, so Issue creation sounds good. Thank
you.

As I stated above, it's a work in progress project, but one day it wil be
ready to be marked with a precious `v1.0.0` tag.

## Configuration

Configuration file:
 - Linux/Mac: `~/.config/todoist-cli/config.ron`
 - Windows: `%APPDATA%\todoist-cli\config\config.ron`

```ron
(
    token: "your todoist api token",
)
```

## Example commands

```bash
# Add new task to inbox (default project)
todoist-cli task new --content="fancy task" --priority=high

# List projects
todoist-cli project list

# List Today's tasks
todoist-cli task list --filter="today"
```

## Library usage

Check `bin/todoist-cli`, I think that's the best way to see how to use it. In
case it's not good enough, here is an example:

```rust
use todoist::Client;
use todoist::api::CreateRequest;
use todoist::enums::Priority;

#[tokio::main]
async fn main() {
    let client = todoist::Client::new(cfg.token);

    let create_request = CreateRequest{
        content: "fancy task".to_string(),
        description: Some("with fancy description".to_string()),
        project_id: None, // It will be the default one: Inbox
        section_id: None,
        parent_id: None,
        order: None,
        labels: None,
        priority: Some(Priority::Urgent),
        due_string: Some("tomorrow 10pm".to_string()),
        due_date: None,
        due_datetime: None,
        due_lang: None,
        assignee_id: None,
    };

    match client.task().create(opts.into()).await {
        Ok(Some(task)) => { println!("{:#?}", task); },
        Ok(None) => { println!("something went wrong"); },
        Err(err) => { println!("error: {}", err); },
    };
}
```
