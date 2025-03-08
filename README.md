# TODO Reminder

TODO Reminder is a command-line tool that scans your codebase for TODO comments with specific formats and generates a report of upcoming and overdue tasks. It helps developers keep track of pending issues and deadlines directly within the code.

## Features

- Parses TODO comments with due dates and owners.
- Finds TODO comments with invalid formats.
- Supports multiple programming languages with language-specific configurations.
- Generates formatted reports with clickable links to code lines.
- Customizable settings via a configuration file.
- Supports output in table or JSON format.

## Installation

### Pre-built Binaries

Download the pre-built binaries from the [Releases](https://github.com/leo108/todo-reminder/releases) page.

### Docker

Pull the Docker image from the Docker registry:

```bash
docker pull leo108/todo-reminder:latest
```

### From Source

Clone the repository and build the project:

```bash
git clone https://github.com/leo108/todo-reminder.git
cd todo-reminder
cargo build --release
```

## Usage

Run the tool by specifying the path to your configuration file:

```bash
todo-reminder config.toml
```

Docker container:

```bash
docker run --rm -t -v $(pwd):/workdir leo108/todo-reminder:latest /workdir/config.toml
```

### Command-Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--check-format-only` | Only check for TODO comments with invalid formats, conflicts with `--check-due-only` | `false` |
| `--check-due-only` | Only check for overdue or due soon TODOs, conflicts with `--check-format-only` | `false` |
| `--due-in=<DAYS>` | Only show TODOs that are due within the specified number of days | `0` |
| `--format=<FORMAT>` | Specify the output format (`table` or `json`) | `table` |
| `--max-comment-length=<LENGTH>` | Truncate comments to the specified maximum length in table format | `100` |
| `--no-tty` | Disable TTY output | `false` |
| `--exit-zero` | Always exit with a status code of 0 | `false` |

## Configuration

Create a `config.toml` file to specify the directories to scan and other settings. An example configuration:

```toml
[parameters]
editor_url = "vscode://file/%%file%%:%%line%%"

[[rules]]
paths = ["src"]
language = "rust"
file_extensions = ["rs"]
```

Check out the [config.example.toml](config.example.toml) file for details.

## Continuous Integration

TODO Reminder can be integrated into your CI/CD pipeline to automatically check for outdated or incorrectly formatted TODO comments. Example configurations are provided for popular CI platforms:

- [GitHub Actions](ci-examples/github-actions.yml)
- [GitLab CI](ci-examples/gitlab-ci.yml)
- [Bitbucket Pipelines](ci-examples/bitbucket-pipelines.yml)

These examples can be used to:
- Run scheduled checks for TODOs (e.g., weekly)
- Check for TODOs on pull/merge requests
- Generate reports and artifacts for tracking

## Supported Languages

| Language    | Config value    | Default file extensions |
|------------|----------------|----------------------|
| Bash       | `bash`        | `["sh"]`              |
| C          | `c`           | `["c", "h"]`          |
| C#         | `c-sharp`     | `["cs", "csx", "cake", "cshtml", "razor"]` |
| C++        | `cpp`         | `["cpp", "hpp", "cc", "hh", "cxx", "hxx"]` |
| CSS        | `css`         | `["css"]`             |
| Go         | `go`          | `["go"]`              |
| Java       | `java`        | `["java"]`            |
| JavaScript | `javascript`  | `["js", "mjs"]`       |
| PHP        | `php`         | `["php"]`             |
| Python     | `python`      | `["py"]`              |
| Ruby       | `ruby`        | `["rb"]`              |
| Rust       | `rust`        | `["rs"]`              |
| TypeScript | `typescript`  | `["ts"]`              |

## TODO Comment Format

The tool looks for TODO comments in the following format:

```
// TODO: YYYY-MM-DD @owner Comment text
```

Example:

```rust
// TODO: 2023-12-31 @alice Refactor this function to improve performance
// TODO not a valid format
```

Table output:

```plaintext
╭──────┬─────────┬────────────┬───────┬──────────────────────────────────────────────────────────────────────────╮
│ Line ┆ Type    ┆ Due Date   ┆ Owner ┆ src/demo.rs                                                              │
╞══════╪═════════╪════════════╪═══════╪══════════════════════════════════════════════════════════════════════════╡
│ 1    ┆ Overdue ┆ 2023-12-31 ┆ alice ┆ // TODO: 2023-12-31 @alice Refactor this function to improve performance │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2    ┆ Format  ┆            ┆       ┆ // TODO not a valid format                                               │
╰──────┴─────────┴────────────┴───────┴──────────────────────────────────────────────────────────────────────────╯
```

JSON output:

```json
[
  {
    "comment": "// TODO: 2023-12-31 @alice Refactor this function to improve performance",
    "due_date": "2023-12-31",
    "file": "src/demo.rs",
    "line": 1,
    "owner": "alice",
    "type": "Overdue"
  },
  {
    "comment": "// TODO not a valid format",
    "file": "src/demo.rs",
    "line": 2,
    "type": "InvalidFormat"
  }
]
```

## Contributing

Contributions are welcome! Please open issues and submit pull requests for improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.