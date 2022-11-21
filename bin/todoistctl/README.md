# Todoist CLI

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

