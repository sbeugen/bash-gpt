mod arguments;
mod gpt_fetch;

use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
    let api_key = env::var(OPENAI_API_KEY).expect(&format!(
        "Environment variable {} has to be set",
        OPENAI_API_KEY
    ));
    let args = arguments::get();

    let mut query = format!(
        "Provide the bash command for doing: {}",
        args.query.join(" ")
    );

    if args.verbose {
        query += " Explain the command."
    } else {
        query += " Omit any additional information and do not explain your answer."
    }

    let command = gpt_fetch::fetch(query.as_str(), api_key).await.unwrap();

    println!("{command}");
}
