use serde_json;

use clap::{App, Arg};
use qqg::extract_links;
use reqwest::header::{HeaderMap, USER_AGENT};

const BASE_URL: &str = "https://html.duckduckgo.com/html/?q=";

fn main() {
    let matches = App::new("qqg")
        .version("0.2.1")
        .about("A small CLI search tool.")
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
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:88.0) Gecko/20100101 Firefox/88.0"
            .parse()
            .unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Unable to create reqwest client");
    let response = client.get(&url).send().expect("Error retrieving response.");

    let result_html = response.text().expect("Failed to read response");

    let links = extract_links(&result_html);

    if json_output {
        let json_str = serde_json::to_string(&links).expect("Failed to serialize to JSON");
        println!("{}", json_str);
        println!("");
    } else if headers_only {
        for link in &links {
            println!("{}", link.title.to_lowercase().trim());
        }
    } else {
        for link in &links {
            println!(
                "## [{}]({})\n\n{}\n\n",
                link.title, link.href, link.description
            );
        }
    }
}
