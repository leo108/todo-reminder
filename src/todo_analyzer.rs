use crate::languages::LanguageConfig;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

#[derive(Debug)]
pub struct TodoItem {
    pub text: String,
    pub due_date: Option<DateTime<Local>>,
    pub owner: Option<String>,
    pub line_number: usize,
    pub is_valid_format: bool,
}

#[derive(Debug)]
pub enum TodoWarning {
    InvalidFormat {
        line_number: usize,
        comment: String,
    },
    Overdue {
        line_number: usize,
        due_date: DateTime<Local>,
        owner: Option<String>,
        comment: String,
    },
    DueSoon {
        line_number: usize,
        due_date: DateTime<Local>,
        owner: Option<String>,
        comment: String,
        days_until_due: i64,
    },
}

impl TodoWarning {
    pub fn line_number(&self) -> usize {
        match self {
            TodoWarning::InvalidFormat { line_number, .. } => *line_number,
            TodoWarning::Overdue { line_number, .. } => *line_number,
            TodoWarning::DueSoon { line_number, .. } => *line_number,
        }
    }
}

pub struct TodoAnalyzer<'config> {
    parser: Parser,
    todo_regex: Regex,
    todo_format_regex: Regex,
    language_configs: &'config HashMap<String, LanguageConfig>,
}

impl<'config> TodoAnalyzer<'config> {
    pub fn new(language_configs: &'config HashMap<String, LanguageConfig>) -> Result<Self> {
        let parser = Parser::new();

        let todo_regex = Regex::new(r"(?i)(TODO|FIXME)(:)?|@todo(:)?")?;
        let todo_format_regex =
            Regex::new(r"(?i)(TODO|FIXME):\s*(\d{4}-\d{2}-\d{2})\s+@(\w+)\s*(.*)")?;

        Ok(Self {
            parser,
            todo_regex,
            todo_format_regex,
            language_configs,
        })
    }

    pub fn set_language(&mut self, language: &str) -> Result<()> {
        if let Some(config) = self.language_configs.get(language) {
            self.parser.set_language(&config.language)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Unsupported language: {}", language))
        }
    }

    pub fn analyze_file(&mut self, file_path: &Path, language: &str) -> Result<Vec<TodoItem>> {
        let content = std::fs::read_to_string(file_path)?;
        let tree = self
            .parser
            .parse(&content, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

        // Get the comment tokens for the specified language
        let comment_queries = self
            .language_configs
            .get(language)
            .map(|config| &config.comment_queries)
            .ok_or_else(|| anyhow::anyhow!("Unsupported language: {}", language))?;

        let mut todos = Vec::new();

        for comment_query in comment_queries {
            let mut cursor = QueryCursor::new();
            let query = Query::new(&self.parser.language().unwrap(), &comment_query)?;
            let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

            while let Some(match_) = matches.next() {
                let comment_node = match_.captures[0].node;
                let comment_text = comment_node.utf8_text(content.as_bytes())?;

                // Check if comment contains TODO
                if self.todo_regex.is_match(comment_text) {
                    let todo_text = comment_text.trim();

                    // Parse TODO format
                    if let Some(captures) = self.todo_format_regex.captures(comment_text) {
                        let due_date = captures
                            .get(2)
                            .map(|m| {
                                NaiveDateTime::parse_from_str(
                                    &format!("{} 00:00:00", m.as_str()),
                                    "%Y-%m-%d %H:%M:%S",
                                )
                                .ok()
                                .map(|dt| Local.from_local_datetime(&dt).unwrap())
                            })
                            .flatten();

                        let owner = captures.get(3).map(|m| m.as_str().to_string());

                        todos.push(TodoItem {
                            text: todo_text.to_string(),
                            due_date,
                            owner,
                            line_number: comment_node.start_position().row + 1,
                            is_valid_format: true,
                        });
                    } else {
                        // Invalid format
                        todos.push(TodoItem {
                            text: todo_text.to_string(),
                            due_date: None,
                            owner: None,
                            line_number: comment_node.start_position().row + 1,
                            is_valid_format: false,
                        });
                    }
                }
            }
        }

        Ok(todos)
    }

    pub fn check_todos(&self, todos: &[TodoItem]) -> Vec<TodoWarning> {
        let now = Local::now();
        let mut warnings = Vec::new();

        for todo in todos {
            if !todo.is_valid_format {
                warnings.push(TodoWarning::InvalidFormat {
                    line_number: todo.line_number,
                    comment: todo.text.clone(),
                });
            } else if let Some(due_date) = todo.due_date {
                let days_until_due = (due_date - now).num_days();

                if due_date < now {
                    warnings.push(TodoWarning::Overdue {
                        line_number: todo.line_number,
                        due_date,
                        owner: todo.owner.clone(),
                        comment: todo.text.clone(),
                    });
                } else {
                    warnings.push(TodoWarning::DueSoon {
                        line_number: todo.line_number,
                        due_date,
                        owner: todo.owner.clone(),
                        comment: todo.text.clone(),
                        days_until_due,
                    });
                }
            }
        }

        warnings
    }
}
