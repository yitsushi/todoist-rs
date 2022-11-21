# Todoist CLI

```bash
cargo install todoistctl
```

### Configuration

Configuration file:
 - Linux/Mac: `~/.config/todoistctl/config.ron`
 - Windows: `%APPDATA%\todoistctl\config\config.ron`

```ron
(
    token: "your todoist api token",
)
```

### Example commands

```bash
# Add new task to inbox (default project)
todoistctl task new --content="fancy task" --priority=high

# List projects
todoistctl project list

# List Today's tasks
todoistctl task list --filter="today"
```
