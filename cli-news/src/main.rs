use std::error::Error;

use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("NEWS_API_KEY")?;

    let url: &str =
        "https://newsapi.org/v2/everything?q=Final%20Fantasy%20XIV&sortBy=popularity&apiKey=";

    let url = format!("{}{}", url, api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
