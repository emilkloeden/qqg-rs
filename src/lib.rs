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

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.href == other.href
            && self.description == other.description
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let html = r#"
        <div id="links" class="results">
        <div class="result results_links results_links_deep web-result">
          <div class="links_main links_deep result__body">
            <!-- This is the visible part -->
      
            <h2 class="result__title">
              <a
                rel="nofollow"
                class="result__a"
                href="https://www.example.com/1"
                >Title 1</a
              >
            </h2>
      
            <div class="result__extras">
              <div class="result__extras__url">
                <span class="result__icon">
                </span>
      
                <a class="result__url" href="https://www.example.com/1">
                  www.example.com/1
                </a>
              </div>
            </div>
      
            <a class="result__snippet" href="https://www.example.com/1"
              >Description 1.</a
            >
      
            <div class="clear"></div>
          </div>
        </div>
      
        <div class="result results_links results_links_deep web-result">
          <div class="links_main links_deep result__body">
            <!-- This is the visible part -->
      
            <h2 class="result__title">
              <a
                rel="nofollow"
                class="result__a"
                href="https://www.example.com/2"
                >Title 2</a
              >
            </h2>
      
            <div class="result__extras">
              <div class="result__extras__url">
                <span class="result__icon">
                </span>
      
                <a class="result__url" href="https://www.example.com/2">
                  www.example.com/2
                </a>
              </div>
            </div>
      
            <a class="result__snippet" href="https://www.example.com/2"
              >Description 2.</a
            >
      
            <div class="clear"></div>
          </div>
        </div>
      
      </div>
      
        "#;
        let expected_links = vec![
            Link::new("Title 1", "https://www.example.com/1", "Description 1."),
            Link::new("Title 2", "https://www.example.com/2", "Description 2."),
        ];

        assert_eq!(extract_links(html), expected_links);
    }

    #[test]
    fn test_link_display() {
        let link = Link::new("Title", "http://example.com", "Description");
        assert_eq!(link.to_string(), "Title (http://example.com)");
    }
}
