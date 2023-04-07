use chatgpt::err::Error;
use chatgpt::prelude::ChatGPT;
use chatgpt::types::CompletionResponse;

pub async fn fetch(query: &str, api_key: String) -> Result<String, Error> {
    let client = ChatGPT::new(api_key)?;
    let response: CompletionResponse = client.send_message(query).await?;

    Ok(response.message().content.clone())
}
