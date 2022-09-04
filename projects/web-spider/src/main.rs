use std::collections::VecDeque;
use std::time::Duration;
use web_spider::{Websites, pull_sites};
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {
    let mut sites = Websites::new();
    let mut todo: VecDeque<String> = VecDeque::new();
    todo.push_back(String::from("https://www.google.com"));
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(7))
        .build()
        .unwrap();

    loop {
        let url = match todo.pop_front() {
            Some(url) => url,
            None => break
        };
        println!("LOG: trying to visit {}", url);
        let content = match client.get(&url).send().await {
            Ok(response) => response.text().await.unwrap(),
            Err(_) => continue
        };
        let urls = sites.append_sites(&pull_sites(&content));
        let allowed = [1000 - todo.len(), 0].into_iter().max().unwrap();
        println!("LOG: {} visited ({} urls found, {} in todo)", url, urls.len(), todo.len()+[urls.len(), allowed].into_iter().min().unwrap());
        todo.append(&mut VecDeque::from(urls.into_iter().take(allowed).collect::<Vec<_>>()))
    }

    println!("{:?}", sites);
}
