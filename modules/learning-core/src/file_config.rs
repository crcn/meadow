use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct FileConfig {
    pub server: ServerConfig,
    pub ai: AiConfig,
    pub curriculum: CurriculumConfig,
    pub prompts: PromptsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub restate_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurriculumConfig {
    pub default_chapter_count: u32,
    pub max_depth: u32,
    pub prefetch_depth: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptsConfig {
    pub dir: String,
}

impl FileConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config from {:?}", path.as_ref()))?;
        toml::from_str(&content).context("Failed to parse config TOML")
    }
}
