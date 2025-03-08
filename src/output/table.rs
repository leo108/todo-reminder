use crate::cli::Cli;
use crate::todo_analyzer::TodoWarning;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Table};
use std::collections::BTreeMap;
use std::env;
use std::path::Path;

fn format_multiline_comment(comment: &str) -> String {
    let lines: Vec<&str> = comment.lines().map(|line| line.trim()).collect();
    lines.join("\n")
}

pub fn print_table(
    warnings_by_file: &BTreeMap<String, Vec<TodoWarning>>,
    cli: &Cli,
    editor_url: &str,
) {
    for (file_path, warnings) in warnings_by_file {
        print_formatted_warnings(file_path, warnings, cli, editor_url);
    }
}

fn print_formatted_warnings(
    file_path: &str,
    warnings: &[TodoWarning],
    cli: &Cli,
    editor_url: &str,
) {
    if warnings.is_empty() {
        return;
    }

    let has_warnings_to_display = warnings.iter().any(|warning| match warning {
        TodoWarning::InvalidFormat { .. } => {
            cli.check_format_only || (!cli.check_due_only && !cli.check_format_only)
        }
        TodoWarning::Overdue { .. } => {
            cli.check_due_only || (!cli.check_due_only && !cli.check_format_only)
        }
        TodoWarning::DueSoon { .. } => {
            cli.check_due_only || (!cli.check_due_only && !cli.check_format_only)
        }
    });

    if !has_warnings_to_display {
        return;
    }

    let first_warning_line = warnings[0].line_number();

    // Get the absolute path for the clickable link
    let absolute_path = if Path::new(file_path).is_relative() {
        if let Ok(current_dir) = env::current_dir() {
            let absolute = current_dir.join(file_path);
            match absolute.to_str() {
                Some(abs_path) => abs_path.to_string(),
                None => file_path.to_string(),
            }
        } else {
            file_path.to_string()
        }
    } else {
        file_path.to_string()
    };

    let clickable_file_link = if cli.no_tty {
        file_path.to_string()
    } else {
        get_clickable_file_link(
            &absolute_path,
            first_warning_line,
            &truncate_file_path(file_path, cli.max_comment_length),
            editor_url,
        )
    };

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    if cli.no_tty {
        table.force_no_tty();
    }
    table.set_header(vec![
        Cell::new("Line")
            .fg(comfy_table::Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Type")
            .fg(comfy_table::Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Due Date")
            .fg(comfy_table::Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Owner")
            .fg(comfy_table::Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new(&clickable_file_link)
            .fg(comfy_table::Color::Cyan)
            .add_attribute(Attribute::Bold),
    ]);

    for warning in warnings {
        match warning {
            TodoWarning::InvalidFormat {
                line_number,
                comment,
                ..
            } => {
                if cli.check_format_only || (!cli.check_due_only && !cli.check_format_only) {
                    table.add_row(vec![
                        Cell::new(if cli.no_tty {
                            line_number.to_string()
                        } else {
                            get_clickable_file_link(
                                file_path,
                                *line_number,
                                &line_number.to_string(),
                                editor_url,
                            )
                        })
                        .fg(comfy_table::Color::Yellow),
                        Cell::new("Format").fg(comfy_table::Color::Magenta),
                        Cell::new(""),
                        Cell::new(""),
                        Cell::new(&truncate_comment(
                            &format_multiline_comment(comment),
                            cli.max_comment_length,
                        )),
                    ]);
                }
            }
            TodoWarning::Overdue {
                line_number,
                due_date,
                owner,
                comment,
                ..
            } => {
                if cli.check_due_only || (!cli.check_due_only && !cli.check_format_only) {
                    table.add_row(vec![
                        Cell::new(if cli.no_tty {
                            line_number.to_string()
                        } else {
                            get_clickable_file_link(
                                file_path,
                                *line_number,
                                &line_number.to_string(),
                                editor_url,
                            )
                        })
                        .fg(comfy_table::Color::Yellow),
                        Cell::new("Overdue").fg(comfy_table::Color::Red),
                        Cell::new(&due_date.format("%Y-%m-%d").to_string())
                            .fg(comfy_table::Color::Red),
                        Cell::new(owner.as_deref().unwrap_or("")),
                        Cell::new(&truncate_comment(
                            &format_multiline_comment(comment),
                            cli.max_comment_length,
                        )),
                    ]);
                }
            }
            TodoWarning::DueSoon {
                line_number,
                due_date,
                owner,
                comment,
                ..
            } => {
                if cli.check_due_only || (!cli.check_due_only && !cli.check_format_only) {
                    table.add_row(vec![
                        Cell::new(if cli.no_tty {
                            line_number.to_string()
                        } else {
                            get_clickable_file_link(
                                file_path,
                                *line_number,
                                &line_number.to_string(),
                                editor_url,
                            )
                        })
                        .fg(comfy_table::Color::Yellow),
                        Cell::new("Due Soon").fg(comfy_table::Color::Yellow),
                        Cell::new(&due_date.format("%Y-%m-%d").to_string())
                            .fg(comfy_table::Color::Yellow),
                        Cell::new(owner.as_deref().unwrap_or("")),
                        Cell::new(&truncate_comment(
                            &format_multiline_comment(comment),
                            cli.max_comment_length,
                        )),
                    ]);
                }
            }
        }
    }

    println!("{}", table);
}

fn truncate_file_path(file_path: &str, max_length: usize) -> String {
    if file_path.len() > max_length {
        format!("...{}", &file_path[file_path.len() - max_length..])
    } else {
        file_path.to_string()
    }
}

fn truncate_comment(comment: &str, max_length: usize) -> String {
    comment
        .lines()
        .map(|line| {
            if line.len() > max_length {
                format!("{}...", &line[..max_length])
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_clickable_file_link(
    file_path: &str,
    line_number: usize,
    display_text: &str,
    editor_url: &str,
) -> String {
    let url = editor_url
        .replace("%%file%%", file_path)
        .replace("%%line%%", &line_number.to_string());
    format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", url, display_text)
}
