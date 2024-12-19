use std::collections::HashMap;
use tree_sitter::Language;

pub struct LanguageConfig {
    pub language: Language,
    pub file_extensions: Vec<&'static str>,
    pub comment_queries: Vec<&'static str>,
}

macro_rules! insert_language_config {
    ($configs:expr, $name:expr, $language:expr, $extensions:expr, $comment_queries:expr) => {
        $configs.insert(
            $name.to_string(),
            LanguageConfig {
                language: $language.into(),
                file_extensions: $extensions,
                comment_queries: $comment_queries,
            },
        );
    };
}

pub fn get_language_configs() -> HashMap<String, LanguageConfig> {
    let mut configs = HashMap::new();

    insert_language_config!(
        configs,
        "bash",
        tree_sitter_bash::LANGUAGE,
        vec!["sh"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "c",
        tree_sitter_c::LANGUAGE,
        vec!["c", "h"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "c-sharp",
        tree_sitter_c_sharp::LANGUAGE,
        vec!["cs", "csx", "cake", "cshtml", "razor"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "cpp",
        tree_sitter_cpp::LANGUAGE,
        vec!["cpp", "hpp", "cc", "hh", "cxx", "hxx"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "css",
        tree_sitter_css::LANGUAGE,
        vec!["css"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "go",
        tree_sitter_go::LANGUAGE,
        vec!["go"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "java",
        tree_sitter_java::LANGUAGE,
        vec!["java"],
        vec!["(line_comment) @comment", "(block_comment) @comment"]
    );
    insert_language_config!(
        configs,
        "javascript",
        tree_sitter_javascript::LANGUAGE,
        vec!["js", "mjs"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "php",
        tree_sitter_php::LANGUAGE_PHP,
        vec!["php"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "python",
        tree_sitter_python::LANGUAGE,
        vec!["py"],
        vec![
            "(comment) @comment",
            "(module . (expression_statement (string) @comment))",
            "(class_definition body: (block . (expression_statement (string) @comment)))",
            "(function_definition body: (block . (expression_statement (string) @comment)))",
        ]
    );
    insert_language_config!(
        configs,
        "ruby",
        tree_sitter_ruby::LANGUAGE,
        vec!["rb"],
        vec!["(comment) @comment"]
    );
    insert_language_config!(
        configs,
        "rust",
        tree_sitter_rust::LANGUAGE,
        vec!["rs"],
        vec!["(line_comment) @comment", "(block_comment) @comment"]
    );
    insert_language_config!(
        configs,
        "typescript",
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        vec!["ts"],
        vec!["(comment) @comment"]
    );

    configs
}
