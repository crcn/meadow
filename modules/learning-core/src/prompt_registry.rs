use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PromptRegistry {
    prompts: HashMap<String, String>,
}

impl PromptRegistry {
    pub fn load(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref();
        let mut prompts = HashMap::new();

        if !dir.exists() {
            anyhow::bail!("Prompts directory does not exist: {:?}", dir);
        }

        for entry in std::fs::read_dir(dir).context("Failed to read prompts directory")? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();

                let content = std::fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read prompt file: {:?}", path))?;

                prompts.insert(name, content);
            }
        }

        tracing::info!("Loaded {} prompt templates", prompts.len());
        Ok(Self { prompts })
    }

    pub fn get(&self, name: &str) -> Result<&str> {
        self.prompts
            .get(name)
            .map(|s| s.as_str())
            .with_context(|| format!("Prompt template not found: {}", name))
    }

    pub fn render(&self, name: &str, vars: &[(&str, &str)]) -> Result<String> {
        let mut template = self.get(name)?.to_string();
        for (key, value) in vars {
            template = template.replace(&format!("{{{{{}}}}}", key), value);
        }
        Ok(template)
    }
}
