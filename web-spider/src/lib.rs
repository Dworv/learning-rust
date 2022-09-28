use std::time::Instant;
use std::collections::HashMap;
use linkify::{LinkFinder, LinkKind};
use url::Url;
use std::collections::HashSet;

/// A website and all the information about it
#[derive(Debug)]
pub struct WebsiteMetadata {
    /// How many times it was found in other sites
    pub frequency: u64,
    /// Last time it was visited
    pub date: Instant
}

impl WebsiteMetadata {
    pub fn new() -> Self {
        Self {
            frequency: 1,
            date: Instant::now()
        }
    }
}

/// A wrapper for a list of `Website`s
#[derive(Debug)]
pub struct Websites {
    inner: HashMap<String, WebsiteMetadata>
}

impl Websites {
    pub fn new() -> Self {
        Self{ inner: HashMap::new() }
    }

    fn add(&mut self, url: &str) -> bool {
        let url = String::from(url);
        if !self.inner.contains_key(&url) {
            self.inner.insert(url, WebsiteMetadata::new());
            true
        } else {
            self.inner.get_mut(&url).unwrap().frequency += 1;
            false
        }
    }

    pub fn append_sites(&mut self, sites: &HashSet<String>) -> Vec<String>{
        let mut todos: Vec<String> = vec![];
        for site in sites.iter() {
            if self.add(&site) {
                todos.push(site.to_string());
            }
        }
        todos
    }
}

pub fn pull_sites(text: &str) -> HashSet<String> {
    let mut scanner = LinkFinder::new();
    scanner.kinds(&[LinkKind::Url]);

    scanner.links(&text as &str)
        .filter_map(|item| Url::parse(item.as_str()).ok())
        .filter(|url| url.scheme() == "https")
        .map(|item| format!("{}://{}", item.scheme(), item.host().unwrap()))
        .collect()
}