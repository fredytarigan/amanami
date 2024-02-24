mod app;

use app::AppConfig;
use colored::*;

pub fn loader(path: &str) -> AppConfig {
    match AppConfig::new(path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "{} unable to prepare configuration with error: {}",
                "Error".red().bold(),
                e
            );
            std::process::exit(1);
        }
    }
}
