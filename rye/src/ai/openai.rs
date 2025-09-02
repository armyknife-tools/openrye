use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::{AIAssistant, AIConfig};

pub struct OpenAIAssistant {
    config: AIConfig,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

impl OpenAIAssistant {
    pub fn new(config: AIConfig) -> Result<Self> {
        Ok(Self { config })
    }
    
    fn make_request(&self, prompt: &str, system: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: self.config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: 0.7,
            max_tokens: 1024,
        };
        
        let mut client = curl::easy::Easy::new();
        let mut response_data = Vec::new();
        
        client.url("https://api.openai.com/v1/chat/completions")?;
        client.post(true)?;
        
        let mut headers = curl::easy::List::new();
        headers.append(&format!("Authorization: Bearer {}", self.config.api_key))?;
        headers.append("Content-Type: application/json")?;
        client.http_headers(headers)?;
        
        let request_body = serde_json::to_string(&request)?;
        client.post_field_size(request_body.len() as u64)?;
        
        {
            use std::io::Read;
            let mut transfer = client.transfer();
            let mut request_bytes = request_body.as_bytes();
            transfer.read_function(move |buf| {
                Ok(request_bytes.read(buf).unwrap_or(0))
            })?;
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        
        let response: OpenAIResponse = serde_json::from_slice(&response_data)?;
        Ok(response.choices.first()
            .ok_or_else(|| anyhow!("No response from OpenAI"))?
            .message.content.clone())
    }
}

impl AIAssistant for OpenAIAssistant {
    fn analyze_error(&self, error: &str, context: &str) -> Result<String> {
        let prompt = format!(
            "Analyze this Python/Rust error and provide a solution:\n\nError: {}\n\nContext: {}",
            error, context
        );
        
        self.make_request(
            &prompt,
            "You are an expert Python and Rust developer helping to debug and fix errors."
        )
    }
    
    fn suggest_dependencies(&self, project_type: &str, description: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Suggest Python dependencies for a {} project: {}\n\nReturn only package names, one per line.",
            project_type, description
        );
        
        let response = self.make_request(
            &prompt,
            "You are an expert Python developer. Suggest only essential, well-maintained packages."
        )?;
        
        Ok(response.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().to_string())
            .collect())
    }
    
    fn generate_code(&self, prompt: &str, language: &str) -> Result<String> {
        let full_prompt = format!(
            "Generate {} code for: {}\n\nProvide only the code without explanations.",
            language, prompt
        );
        
        self.make_request(
            &full_prompt,
            &format!("You are an expert {} developer. Generate clean, idiomatic code.", language)
        )
    }
    
    fn optimize_code(&self, code: &str, language: &str) -> Result<String> {
        let prompt = format!(
            "Optimize this {} code for performance:\n\n```{}\n{}\n```",
            language, language, code
        );
        
        self.make_request(
            &prompt,
            "You are an expert developer. Optimize for performance while maintaining readability."
        )
    }
}