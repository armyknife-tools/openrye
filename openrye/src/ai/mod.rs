// AI Module for OpenRye - Intelligent Python Development Assistance
// Supports multiple AI providers: OpenAI (active) and Anthropic Claude (placeholder)

pub mod code_reviewer;
pub mod pr_generator;
pub mod security_audit;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::env;

// TODO-TEACHING: Trait-based design for multiple AI providers
#[async_trait::async_trait]
pub trait AIProvider: Send + Sync {
    /// Generate a response from the AI model
    async fn generate(&self, prompt: &str, context: Option<&str>) -> Result<String>;
    
    /// Analyze code and suggest improvements
    async fn analyze_code(&self, code: &str) -> Result<CodeAnalysis>;
    
    /// Suggest dependencies based on project description
    async fn suggest_dependencies(&self, description: &str) -> Result<Vec<String>>;
    
    /// Fix errors with AI assistance
    async fn fix_error(&self, error: &str, context: &str) -> Result<String>;
}

// TODO-IMPROVEMENT: Add more analysis types
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub issues: Vec<Issue>,
    pub suggestions: Vec<String>,
    pub performance_tips: Vec<String>,
    pub security_warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub message: String,
    pub line: Option<usize>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

// OpenAI Provider Implementation
pub struct OpenAIProvider {
    api_key: String,
    model: String,
    temperature: f32,
}

impl OpenAIProvider {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;
        
        Ok(Self {
            api_key,
            model: "gpt-5".to_string(), // Default to GPT-5 - 8x cheaper and better!
            temperature: 0.7,
        })
    }
    
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
    
    // TODO-FEATURE: Allow using GPT-5 when available
    pub fn use_gpt5(mut self) -> Self {
        self.model = "gpt-5".to_string(); // Ready for GPT-5
        self
    }
}

#[async_trait::async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str, context: Option<&str>) -> Result<String> {
        let client = reqwest::Client::new();
        
        let mut messages = vec![];
        
        // System message for context
        messages.push(serde_json::json!({
            "role": "system",
            "content": "You are an AI assistant for OpenRye, a Python and Rust development platform. Help with dependency management, code analysis, and project setup."
        }));
        
        // Add context if provided
        if let Some(ctx) = context {
            messages.push(serde_json::json!({
                "role": "system",
                "content": format!("Context: {}", ctx)
            }));
        }
        
        // User prompt
        messages.push(serde_json::json!({
            "role": "user",
            "content": prompt
        }));
        
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": self.model,
                "messages": messages,
                "temperature": self.temperature,
                "max_tokens": 2000,
            }))
            .send()
            .await
            .context("Failed to send request to OpenAI")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("OpenAI API error: {}", error_text);
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format from OpenAI"))
            .map(|s| s.to_string())
    }
    
    async fn analyze_code(&self, code: &str) -> Result<CodeAnalysis> {
        let prompt = format!(
            "Analyze this Python code and provide:\n\
            1. Issues (bugs, errors)\n\
            2. Improvement suggestions\n\
            3. Performance tips\n\
            4. Security warnings\n\n\
            Code:\n```python\n{}\n```\n\n\
            Respond in JSON format with keys: issues, suggestions, performance_tips, security_warnings",
            code
        );
        
        let response = self.generate(&prompt, None).await?;
        
        // TODO-IMPROVEMENT: Better JSON parsing with error handling
        serde_json::from_str(&response)
            .context("Failed to parse AI response as CodeAnalysis")
    }
    
    async fn suggest_dependencies(&self, description: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Based on this project description, suggest Python dependencies:\n\
            Description: {}\n\n\
            Provide a list of recommended packages with versions, one per line.\n\
            Format: package==version or package>=version",
            description
        );
        
        let response = self.generate(&prompt, None).await?;
        
        Ok(response
            .lines()
            .filter(|line| !line.is_empty() && line.contains("==") || line.contains(">="))
            .map(|s| s.to_string())
            .collect())
    }
    
    async fn fix_error(&self, error: &str, context: &str) -> Result<String> {
        let prompt = format!(
            "I encountered this error in my Python project:\n\
            Error: {}\n\n\
            Context:\n{}\n\n\
            Provide a solution to fix this error.",
            error, context
        );
        
        self.generate(&prompt, None).await
    }
}

// Anthropic Claude Provider (Placeholder Implementation)
pub struct ClaudeProvider {
    api_key: String,
    model: String,
}

impl ClaudeProvider {
    pub fn new() -> Result<Self> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY environment variable not set")?;
        
        Ok(Self {
            api_key,
            model: "claude-3-opus-20240229".to_string(), // Latest Claude model
        })
    }
}

#[async_trait::async_trait]
impl AIProvider for ClaudeProvider {
    async fn generate(&self, prompt: &str, context: Option<&str>) -> Result<String> {
        // TODO-FEATURE: Implement when API credits available
        let client = reqwest::Client::new();
        
        let mut system_prompt = "You are an AI assistant for OpenRye, a Python and Rust development platform.".to_string();
        if let Some(ctx) = context {
            system_prompt.push_str(&format!(" Context: {}", ctx));
        }
        
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": 2000,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }],
                "system": system_prompt,
            }))
            .send()
            .await
            .context("Failed to send request to Anthropic")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Anthropic API error: {}", error_text);
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format from Anthropic"))
            .map(|s| s.to_string())
    }
    
    async fn analyze_code(&self, code: &str) -> Result<CodeAnalysis> {
        // TODO-FEATURE: Implement Claude-specific code analysis
        let prompt = format!(
            "Analyze this Python code and provide a JSON response with:\n\
            - issues: array of {{severity, message, line, suggestion}}\n\
            - suggestions: array of improvement suggestions\n\
            - performance_tips: array of performance optimization tips\n\
            - security_warnings: array of security concerns\n\n\
            Code:\n```python\n{}\n```",
            code
        );
        
        let response = self.generate(&prompt, None).await?;
        serde_json::from_str(&response)
            .context("Failed to parse Claude response as CodeAnalysis")
    }
    
    async fn suggest_dependencies(&self, description: &str) -> Result<Vec<String>> {
        // TODO-FEATURE: Implement Claude-specific dependency suggestions
        let prompt = format!(
            "Suggest Python package dependencies for this project:\n{}\n\n\
            List packages with versions (package==version), one per line.",
            description
        );
        
        let response = self.generate(&prompt, None).await?;
        Ok(response.lines().map(|s| s.to_string()).collect())
    }
    
    async fn fix_error(&self, error: &str, context: &str) -> Result<String> {
        // TODO-FEATURE: Implement Claude-specific error fixing
        self.generate(
            &format!("Fix this Python error:\n{}\n\nContext:\n{}", error, context),
            None
        ).await
    }
}

// Factory function to create AI provider based on configuration
pub fn create_ai_provider() -> Result<Box<dyn AIProvider>> {
    // Check which API key is available
    if env::var("OPENAI_API_KEY").is_ok() {
        Ok(Box::new(OpenAIProvider::new()?))
    } else if env::var("ANTHROPIC_API_KEY").is_ok() {
        Ok(Box::new(ClaudeProvider::new()?))
    } else {
        bail!("No AI provider configured. Set either OPENAI_API_KEY or ANTHROPIC_API_KEY")
    }
}

// High-level AI functions for OpenRye commands
pub mod helpers {
    use super::*;
    
    /// Suggest project structure based on description
    pub async fn suggest_project_structure(description: &str) -> Result<String> {
        let provider = create_ai_provider()?;
        let prompt = format!(
            "Create a Python project structure for: {}\n\
            Include folder structure, main files, and configuration.",
            description
        );
        provider.generate(&prompt, None).await
    }
    
    /// Optimize dependencies for a project
    pub async fn optimize_dependencies(current_deps: Vec<String>) -> Result<Vec<String>> {
        let provider = create_ai_provider()?;
        let prompt = format!(
            "Optimize these Python dependencies:\n{}\n\n\
            Suggest better alternatives, version updates, or removals.",
            current_deps.join("\n")
        );
        
        let response = provider.generate(&prompt, None).await?;
        Ok(response.lines().map(|s| s.to_string()).collect())
    }
    
    /// Generate test cases for code
    pub async fn generate_tests(code: &str) -> Result<String> {
        let provider = create_ai_provider()?;
        let prompt = format!(
            "Generate pytest test cases for this Python code:\n```python\n{}\n```",
            code
        );
        provider.generate(&prompt, None).await
    }
    
    /// Convert Python code to Rust (for hot path optimization)
    pub async fn python_to_rust(python_code: &str) -> Result<String> {
        let provider = create_ai_provider()?;
        let prompt = format!(
            "Convert this Python code to Rust using PyO3 bindings:\n```python\n{}\n```\n\n\
            Provide complete Rust code with PyO3 annotations.",
            python_code
        );
        provider.generate(&prompt, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ai_provider_creation() {
        // Test will pass if either API key is set
        let result = create_ai_provider();
        if env::var("OPENAI_API_KEY").is_err() && env::var("ANTHROPIC_API_KEY").is_err() {
            assert!(result.is_err());
        }
    }
    
    // TODO-IMPROVEMENT: Add mock tests for AI providers
}