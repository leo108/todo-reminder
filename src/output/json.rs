use crate::todo_analyzer::TodoWarning;
use serde_json::json;
use std::collections::BTreeMap;

pub fn print_json(warnings_by_file: &BTreeMap<String, Vec<TodoWarning>>) {
    let mut json_warnings = Vec::new();

    for (file_path, warnings) in warnings_by_file {
        for warning in warnings {
            let warning_json = match warning {
                TodoWarning::InvalidFormat {
                    line_number,
                    comment,
                } => json!({
                    "file": file_path,
                    "line": line_number,
                    "type": "InvalidFormat",
                    "comment": comment,
                }),
                TodoWarning::Overdue {
                    line_number,
                    due_date,
                    owner,
                    comment,
                } => json!({
                    "file": file_path,
                    "line": line_number,
                    "type": "Overdue",
                    "due_date": due_date.format("%Y-%m-%d").to_string(),
                    "owner": owner,
                    "comment": comment,
                }),
                TodoWarning::DueSoon {
                    line_number,
                    due_date,
                    owner,
                    comment,
                    days_until_due,
                } => json!({
                    "file": file_path,
                    "line": line_number,
                    "type": "DueSoon",
                    "due_date": due_date.format("%Y-%m-%d").to_string(),
                    "owner": owner,
                    "comment": comment,
                    "days_until_due": days_until_due,
                }),
            };
            json_warnings.push(warning_json);
        }
    }

    println!("{}", serde_json::to_string_pretty(&json_warnings).unwrap());
}
