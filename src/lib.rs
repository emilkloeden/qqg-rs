use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Predicate};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use url::Url;

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

fn get_href_from_uddg_param(n: Node) -> String {
    let scheme = "https:";
    let href = n.attr("href").unwrap();
    let url_string = format!("{}{}", scheme, href);

    let url = Url::parse(&url_string).unwrap();

    let mut query_pairs = url.query_pairs();

    let href = loop {
        match query_pairs.next() {
            Some((key, value)) => {
                if key == "uddg" {
                    return value.to_string();
                }
            }
            None => break "".to_string(),
        }
    };
    href
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
            println!("{:#?}", title);
            let href = node
                .find(Class("result__snippet"))
                .next()
                .map(|n| get_href_from_uddg_param(n))
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
