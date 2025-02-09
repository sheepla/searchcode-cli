use crate::types::SearchResult;
use colored::Colorize;
use serde_json::Value;

pub fn render(data: &SearchResult) -> eyre::Result<()> {
    for item in data.results.iter() {
        println!(
            "\n | {} at {}\n | on {} | {} | {} Lines\n | {}",
            &item.filename.bold(),
            &item.location.cyan(),
            &item.repo.blue(),
            &item.language.bold(),
            &item.linescount.to_string().bold(),
            &item.url.blue().underline(),
        );

        if let Value::Object(obj) = &item.lines {
            for (key, value) in obj {
                let line_number = str::parse::<i32>(&key)?;
                if let Value::String(code_line) = value {
                    println!("    {:>5} {}", line_number.to_string().dimmed(), code_line);
                }
            }
        }
    }

    Ok(())
}
