use select::document::Document;
use select::predicate::{Class, Predicate};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub href: String,
    pub description: String,
}

impl Link {
    fn new(title: &str, href: &str, description: &str) -> Link {
        Link {
            title: title.trim().to_string(),
            href: href.trim().to_string(),
            description: description.trim().to_string(),
        }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({})", self.title, self.href)
    }
}

pub fn extract_links(result_html: &str) -> Vec<Link> {
    let document = Document::from(result_html);
    let soup =
        document.find(Class("links_main").and(Class("links_deep").and(Class("result__body"))));
    let links = soup
        .map(|node| {
            let title = node
                .find(Class("result__title"))
                .next()
                .map(|n| n.text())
                .unwrap()
                .trim()
                .to_string();
            let href = node
                .find(Class("result__snippet"))
                .next()
                .map(|n| n.attr("href").unwrap().to_string())
                .unwrap();
            let description = node
                .find(Class("result__snippet"))
                .next()
                .map(|n| n.text())
                .unwrap()
                .trim()
                .to_string();

            let link = Link::new(&title, &href, &description);
            link
        })
        .collect();
    links
}
