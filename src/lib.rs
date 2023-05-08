use select::document::Document;
use select::predicate::{Class, Name, Predicate};
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
    let links = document
        .find(Name("a").and(Class("result__url")))
        .filter_map(|n| {
            let href = n.attr("href")?;
            let title = n
                .find(Name("strong"))
                .next()
                .map(|n| n.text())
                .unwrap_or_else(|| n.text());
            let description = n
                .parent()
                .and_then(|p| p.find(Class("result__snippet")).next())
                .map(|n| n.text())
                .unwrap_or_default();
            Some(Link::new(&title, href, &description))
        })
        .collect();
    links
}
