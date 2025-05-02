use reqwest::Client;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize,Serialize};
use std::io::stdin;


#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    let client = Client::new();
    let mut conversation = Conversation {
        messages: Vec::new(),
    };

    println!("\n Hello User, what do you wanna discuss today?\n");

    loop {
        println!("You:");
        let mut question = String::new();
        stdin().read_line(&mut question).map_err(|e| e.to_string())?;
        let question = question.trim();

        if question.eq_ignore_ascii_case("exit") {
            break;
        }

        conversation.messages.push(Message {
            role: "user".to_string(),
            content: question.to_string(),
        });

        let response = ask_openai(&client, &mut conversation).await?;
        println!("\nTobi AI: {}", response);
    }

    Ok(())
}

async fn ask_openai(client: &Client, conversation: &mut Conversation) -> Result<String, String> {
    let request_body = OpenAIChatRequest {
        model: "gpt-4.1".to_string(),
        messages: conversation.messages.clone(),
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")),
        )
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_body = response
        .json::<OpenAIChatResponse>()
        .await
        .map_err(|e| e.to_string())?;


    if let Some(choice) = response_body.choices.last() {
        conversation.messages.push(Message {
            role: "assistant".to_string(),
            content: choice.message.content.clone(),
        });
        Ok(choice.message.content.to_string())
    } else {
        Err("No response from AI".to_string())
    }
}