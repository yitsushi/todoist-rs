# Todolist API crate

```bash
cargo add libtodoist
```

### Library usage

Check [bin/todoistctl](https://github.com/yitsushi/todoist-rs/tree/main/bin/todoistctl),
I think that's the best way to see how to use it. In case it's not good enough,
here is an example:

```rust
use libtodoist::Client;
use libtodoist::api::CreateRequest;
use libtodoist::enums::Priority;

#[tokio::main]
async fn main() {
    let client = libtodoist::Client::new(cfg.token);

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
