use serde::{Deserialize, Serialize};

/// Configuration for the chat completion endpoint.
pub struct AiConfig {
    pub endpoint: String,
    pub token: String,
    pub model: String,
}

impl AiConfig {
    /// Build config from environment variables:
    /// - `AI_ENDPOINT` (default: `https://api.openai.com/v1/chat/completions`)
    /// - `AI_TOKEN` (required)
    /// - `AI_MODEL` (default: `gpt-4`)
    pub fn from_env() -> Result<Self, String> {
        let endpoint = std::env::var("AI_ENDPOINT")
            .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".into());
        let token = std::env::var("AI_TOKEN")
            .map_err(|_| "AI_TOKEN environment variable is required".to_string())?;
        let model = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4".into());
        Ok(Self {
            endpoint,
            token,
            model,
        })
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

/// Send a chat completion request and return the assistant's response text.
pub fn chat_completion(
    config: &AiConfig,
    system: &str,
    user: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system".into(),
                content: system.into(),
            },
            Message {
                role: "user".into(),
                content: user.into(),
            },
        ],
    };

    let resp = client
        .post(&config.endpoint)
        .header("Authorization", format!("Bearer {}", config.token))
        .json(&request)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(format!("API error {status}: {body}").into());
    }

    let chat_resp: ChatResponse = resp.json()?;
    let content = chat_resp
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    Ok(content)
}
