use crate::types::SearchResult;
use colored::Colorize;
use serde_json::Value;

pub fn render(data: &SearchResult) -> eyre::Result<()> {
    for item in data.results.iter() {
        println!(
            "\n | {} on {}\n | at {}\n | {} | {} Lines\n | {}",
            &item.filename.bold(),
            &item.location.cyan(),
            &item.repo.blue(),
            &item.language,
            &item.linescount,
            &item.url.blue().underline(),
        );

        if let Value::Object(obj) = &item.lines {
            for (key, value) in obj {
                println!("    {} {}", key.dimmed(), value);
            }
        }

    }

    Ok(())
}
