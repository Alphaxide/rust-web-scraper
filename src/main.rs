use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use tokio::task;

async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_headlines(html: &str) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h1, h2, h3").unwrap(); // Select headlines (h1, h2, h3)
    
    for element in document.select(&selector) {
        let headline = element.text().collect::<Vec<_>>().concat();
        println!("Headline: {}", headline);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = vec![
        "https://quiver.vercel.app",     
    ];

    let mut tasks = vec![];

    for url in urls {
        // Spawn a task for each URL
        let task = task::spawn(async move {
            match fetch_html(url).await {
                Ok(html) => {
                    println!("Scraping headlines from: {}", url);
                    parse_headlines(&html);
                }
                Err(e) => eprintln!("Failed to fetch {}: {}", url, e),
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}
