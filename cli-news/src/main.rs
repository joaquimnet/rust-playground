use std::error::Error;

use serde::{Deserialize, Serialize};
use colour::{dark_green, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://newsapi.org/v2/everything?q=Final%20Fantasy%20XIV&sortBy=popularity&apiKey=API_KEY_GOES_HERE";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
