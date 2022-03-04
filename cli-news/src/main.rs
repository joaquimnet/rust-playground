mod theme;

use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for article in &articles.articles {
        theme.print_text(&format!("**{}**\n", article.title));
        theme.print_text(&format!("- *{}*\n", article.url));
        theme.print_text("---")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("NEWS_API_KEY")?;

    let url: &str =
        "https://newsapi.org/v2/everything?q=Final%20Fantasy%20XIV&sortBy=popularity&apiKey=";

    let url = format!("{}{}", url, api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
