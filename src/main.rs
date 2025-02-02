use clap::Parser;

mod api;
mod renderer;
mod types;

#[derive(Parser)]
struct Args {
    query: String,

    #[arg(short, long)]
    language_code: Option<u32>,

    #[arg(short, long)]
    page: Option<u32>,

    #[arg(short, long)]
    max_pages: Option<u32>,

    #[arg(long)]
    min_lines: Option<String>,

    #[arg(long)]
    max_lines: Option<String>,

    #[arg(long)]
    provider_code: Option<u32>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let param = api::SearchParamBuilder::default()
        .query(args.query)
        .language_code(args.language_code)
        .page(args.page)
        .max_pages(args.max_pages)
        .min_lines(args.min_lines)
        .max_lines(args.max_lines)
        .provider_code(args.provider_code)
        .build()?;

    let client = api::ApiClient(reqwest::Client::new());
    let search_result = client.search(&param).await?;

    renderer::render(&search_result)?;

    Ok(())
}
