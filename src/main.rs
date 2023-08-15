use dotenv::dotenv;
use reqwest::Client;
use tokio;

use std::env;

struct Config {
    indeed: String,
    cryptocurrencyjobs: String,
}

const SOLIDITY_JOBS: &str = "Solidity Jobs";
// https://www.indeed.com/jobs?q=solidity&l=&from=searchOnHP&vjk=4ad0698e90685347

#[tokio::main]
async fn main() {
    dotenv().ok();

    let indeed_search_url = env::var("INDEED_SEARCH").expect("ENV INDEED_SEARCH var not found");

    let formatted_url = format_search_url(&indeed_search_url, SOLIDITY_JOBS);

    println!("{}", formatted_url);

    let client = Client::new();
    let response = client.get(formatted_url).send();

    match response.await {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }
}

fn format_search_url(partial_url: &str, search_keywords: &str) -> String {
    let split_partial_url: Vec<String> = partial_url.split("{}").map(String::from).collect();

    let mut split_search_keywords: Vec<String> =
        search_keywords.split(" ").map(String::from).collect();

    // Need to append `+` to all all words in the search text, excluding the first word
    // Example: `Node.js Engineer` => `Node.js` `+Engineer`
    split_search_keywords
        .iter_mut()
        .skip(1)
        .for_each(|item| *item = format!("+{}", item));

    let search_keywords = split_search_keywords.join("");

    // split_partial_url will always have a length of 2,
    // hence we can index the 2 elements (0 and 1)
    format!(
        "{}{}{}",
        split_partial_url[0], search_keywords, split_partial_url[1]
    )
}
