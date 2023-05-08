use serde_json;

use clap::{App, Arg};
use qqg::extract_links;

const BASE_URL: &str = "https://html.duckduckgo.com/html/?q=";

fn main() {
    let matches = App::new("ddg-search")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Search DuckDuckGo from the command line")
        .arg(
            Arg::with_name("query")
                .value_name("QUERY")
                .required(true)
                .help("The search query"),
        )
        .arg(
            Arg::with_name("json")
                .short('j')
                .long("json")
                .help("Output results as JSON"),
        )
        .arg(
            Arg::with_name("headers")
                .short('H')
                .long("headers")
                .help("Output only the headers of the search results"),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let json_output = matches.is_present("json");
    let headers_only = matches.is_present("headers");

    let url = format!("{}{}", BASE_URL, query);
    let response = reqwest::blocking::get(&url).expect("Failed to fetch results");

    let result_html = response.text().expect("Failed to read response");
    // let result_html = dbg!(result_html);

    let links = extract_links(&result_html);

    if json_output {
        let json_str = serde_json::to_string(&links).expect("Failed to serialize to JSON");
        println!("{}", json_str);
    } else if headers_only {
        for link in &links {
            println!("{}", link.title.to_lowercase().trim());
        }
    } else {
        for link in &links {
            println!("## [{}]({})\n\n{}", link.title, link.href, link.description);
        }
    }
}
