mod cli;
mod config;
mod languages;
mod output;
mod todo_analyzer;

use clap::Parser;
use cli::Cli;
use config::Config;
use languages::get_language_configs;
use output::{print_json, print_table};
use std::collections::BTreeMap;
use todo_analyzer::{TodoAnalyzer, TodoWarning};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.validate().map_err(|e| anyhow::anyhow!(e))?;

    let config = Config::from_file(&cli.config_file_path)?;

    let language_configs = get_language_configs();
    let mut analyzer = TodoAnalyzer::new(&language_configs)?;
    let mut warnings_by_file: BTreeMap<String, Vec<TodoWarning>> = BTreeMap::new();
    let config_file_dir = std::path::Path::new(&cli.config_file_path)
        .parent()
        .unwrap();
    let current_dir = std::env::current_dir()?;
    let abs_config_file_dir = if !config_file_dir.is_absolute() {
        current_dir.join(&config_file_dir)
    } else {
        config_file_dir.to_path_buf().clone()
    };

    for rule in &config.rules {
        analyzer.set_language(&rule.language)?;

        let config = match language_configs.get(&rule.language) {
            Some(config) => config,
            None => continue,
        };

        let file_extensions = rule.file_extensions.clone().unwrap_or_else(|| {
            config
                .file_extensions
                .iter()
                .map(|s| s.to_string())
                .collect()
        });

        for rule_path in &rule.paths {
            let abs_path = abs_config_file_dir.join(rule_path);

            if !abs_path.exists() || !abs_path.is_dir() {
                eprintln!(
                    "Directory does not exist or not a folder: {}",
                    abs_path.display()
                );
                continue;
            }

            for entry in WalkDir::new(&abs_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                let extension = path.extension().and_then(|ext| ext.to_str());

                if extension.is_none() {
                    continue;
                }
                let ext = extension.unwrap();

                if !file_extensions.contains(&ext.to_string()) {
                    continue;
                }

                match analyzer.analyze_file(path, &rule.language) {
                    Ok(todos) => {
                        let mut warnings = analyzer.check_todos(&todos, &cli);
                        // sort warnings by line number
                        warnings.sort_by_key(|w| w.line_number());

                        let relative_path = path
                            .strip_prefix(&abs_config_file_dir)?
                            .display()
                            .to_string();
                        warnings_by_file
                            .entry(relative_path)
                            .or_insert_with(Vec::new)
                            .extend(warnings);
                    }
                    Err(e) => {
                        eprintln!("Error analyzing file {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    match cli.format.as_str() {
        "json" => print_json(&warnings_by_file),
        _ => print_table(&warnings_by_file, &cli, &config.parameters.editor_url),
    }

    if !cli.exit_zero && !warnings_by_file.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
