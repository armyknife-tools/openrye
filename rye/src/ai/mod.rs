use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;

pub mod anthropic;
pub mod openai;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    Anthropic,
    OpenAI,
}

#[derive(Debug, Clone)]
pub struct AIConfig {
    pub provider: AIProvider,
    pub api_key: String,
    pub model: String,
}

impl AIConfig {
    pub fn from_env() -> Result<Self> {
        // Check for Anthropic API key first
        if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
            return Ok(Self {
                provider: AIProvider::Anthropic,
                api_key,
                model: env::var("ANTHROPIC_MODEL")
                    .unwrap_or_else(|_| "claude-3-opus-20240229".to_string()),
            });
        }
        
        // Check for OpenAI API key
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            return Ok(Self {
                provider: AIProvider::OpenAI,
                api_key,
                model: env::var("OPENAI_MODEL")
                    .unwrap_or_else(|_| "gpt-4-turbo-preview".to_string()),
            });
        }
        
        Err(anyhow!(
            "No AI provider configured. Set ANTHROPIC_API_KEY or OPENAI_API_KEY environment variable."
        ))
    }
}

pub trait AIAssistant {
    fn analyze_error(&self, error: &str, context: &str) -> Result<String>;
    fn suggest_dependencies(&self, project_type: &str, description: &str) -> Result<Vec<String>>;
    fn generate_code(&self, prompt: &str, language: &str) -> Result<String>;
    fn optimize_code(&self, code: &str, language: &str) -> Result<String>;
}

pub fn get_ai_assistant() -> Result<Box<dyn AIAssistant>> {
    let config = AIConfig::from_env()?;
    
    match config.provider {
        AIProvider::Anthropic => Ok(Box::new(anthropic::AnthropicAssistant::new(config)?)),
        AIProvider::OpenAI => Ok(Box::new(openai::OpenAIAssistant::new(config)?)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
    pub optimizations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyRecommendation {
    pub package: String,
    pub version: String,
    pub reason: String,
}