use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub config_file_path: String,

    #[clap(long, conflicts_with = "check_due_only")]
    pub check_format_only: bool,

    #[clap(long, conflicts_with = "check_format_only")]
    pub check_due_only: bool,

    #[clap(long, default_value = "0")]
    pub due_after: u32,

    #[clap(long, default_value = "100")]
    pub max_comment_length: usize,

    #[clap(long, default_value = "table")]
    pub format: String,

    #[clap(long)]
    pub no_tty: bool,

    #[clap(long)]
    pub exit_zero: bool,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        if self.format != "table" && self.format != "json" {
            Err("Invalid format. Supported formats are 'table' and 'json'.".to_string())
        } else {
            Ok(())
        }
    }
}
