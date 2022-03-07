mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for article in articles {
        theme.print_text(&format!("**{}**\n", article.title()));
        theme.print_text(&format!("- *{}*\n", article.url()));
        theme.print_text("---")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("NEWS_API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);

    let newspi_response = newsapi.fetch_async().await?;

    render_articles(newspi_response.articles());

    Ok(())
}
