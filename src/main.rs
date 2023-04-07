mod arguments;
mod gpt_fetch;

use std::env;

#[tokio::main]
async fn main() {
    const OPEN_AI_API_KEY: &str = "OPEN_AI_API_KEY";
    let api_key = env::var(OPEN_AI_API_KEY).expect(&format!(
        "Environment variable {} has to be set",
        OPEN_AI_API_KEY
    ));
    let args = arguments::get();

    let mut query = format!(
        "Provide the bash command for doing: {}",
        args.query
    );

    if args.verbose {
        query += " Explain the command."
    } else {
        query += " Omit any additional information and do not explain your answer."
    }

    let command = gpt_fetch::fetch(query.as_str(), api_key).await.unwrap();

    println!("{command}");
}
