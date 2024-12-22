use clap::Parser;
use reqwest::{Client, Method, Url};
use std::error::Error;
use std::time::Duration;

const WTTR_URL: &str = "https://wttr.in";
const OPTIONS: &str = "AFq0mM";

fn create_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    Ok(client)
}

async fn send_request(client: &Client, url: Url, method: Method) -> Result<String, Box<dyn Error>> {
    let response = client.request(method, url).send().await?;
    let status = response.status();
    let url_str = response.url().to_string();

    if !status.is_success() {
        return Err(format!("Request to {} failed with status: {}", url_str, status).into());
    }

    let text = response.text().await?;
    Ok(text)
}
fn build_url(location: &str, lang: &str, options: &str) -> Url {
    format!("{}/{}?{}&lang={}", WTTR_URL, location, options, lang)
        .parse()
        .unwrap()
}

fn display_response(response_body: String) {
    println!("{}\n", response_body);
}

#[derive(Parser, Debug)]
#[command(
    name = "wttr-cli",
    about = "Fetch weather information from wttr.in",
    version,
    author
)]
struct Cli {
    #[arg(
        default_value = "",
        help = "City/location names to get the weather for."
    )]
    locations: Vec<String>,

    #[arg(
        short,
        long,
        default_value = "en",
        help = "Specify the language (e.g., ru, fr, de)."
    )]
    lang: String,
    #[arg(
        short,
        long,
        default_value = OPTIONS,
        help = "Specify the options to use. Refer to https://wttr.in/:help for more information. A and F are recommended."
    )]
    options: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let locations = args.locations;
    let lang = args.lang.as_str();
    let options = args.options.as_str();

    let client = create_client()?;

    for location in locations {
        let url = build_url(&location, lang, options);
        let response_body = send_request(&client, url, Method::GET).await?;
        display_response(response_body);
    }
    Ok(())
}
