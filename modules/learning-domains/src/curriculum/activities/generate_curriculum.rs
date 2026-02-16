use anyhow::{Context, Result};
use learning_core::types::CurriculumOutline;
use learning_core::ServerDeps;

pub async fn generate_curriculum_outline(
    deps: &ServerDeps,
    topic: &str,
) -> Result<CurriculumOutline> {
    let chapter_count = deps.file_config.curriculum.default_chapter_count;
    let prompt = deps.prompts.render(
        "generate_curriculum",
        &[
            ("topic", topic),
            ("chapter_count", &chapter_count.to_string()),
        ],
    )?;

    let response = call_ai(deps, &prompt).await?;

    // Extract JSON from the response (handle markdown code blocks)
    let json_str = extract_json(&response)?;

    let outline: CurriculumOutline =
        serde_json::from_str(&json_str).context("Failed to parse AI response as CurriculumOutline")?;

    Ok(outline)
}

async fn call_ai(deps: &ServerDeps, prompt: &str) -> Result<String> {
    let provider = &deps.file_config.ai.provider;
    let model = &deps.file_config.ai.model;
    let max_tokens = deps.file_config.ai.max_tokens;

    match provider.as_str() {
        "anthropic" => call_anthropic(deps, model, max_tokens, prompt).await,
        "openai" => call_openai(deps, model, max_tokens, prompt).await,
        _ => anyhow::bail!("Unknown AI provider: {}", provider),
    }
}

async fn call_anthropic(
    deps: &ServerDeps,
    model: &str,
    max_tokens: u32,
    prompt: &str,
) -> Result<String> {
    let api_key = deps
        .config
        .anthropic_api_key
        .as_ref()
        .context("ANTHROPIC_API_KEY not set")?;

    let body = serde_json::json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    let resp = deps
        .http_client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .context("Failed to call Anthropic API")?;

    let status = resp.status();
    let resp_body: serde_json::Value = resp.json().await.context("Failed to parse Anthropic response")?;

    if !status.is_success() {
        anyhow::bail!("Anthropic API error ({}): {}", status, resp_body);
    }

    let text = resp_body["content"][0]["text"]
        .as_str()
        .context("No text in Anthropic response")?
        .to_string();

    Ok(text)
}

async fn call_openai(
    deps: &ServerDeps,
    model: &str,
    max_tokens: u32,
    prompt: &str,
) -> Result<String> {
    let api_key = deps
        .config
        .openai_api_key
        .as_ref()
        .context("OPENAI_API_KEY not set")?;

    let body = serde_json::json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });

    let resp = deps
        .http_client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .context("Failed to call OpenAI API")?;

    let status = resp.status();
    let resp_body: serde_json::Value = resp.json().await.context("Failed to parse OpenAI response")?;

    if !status.is_success() {
        anyhow::bail!("OpenAI API error ({}): {}", status, resp_body);
    }

    let text = resp_body["choices"][0]["message"]["content"]
        .as_str()
        .context("No content in OpenAI response")?
        .to_string();

    Ok(text)
}

fn extract_json(text: &str) -> Result<String> {
    // Try to find JSON in code blocks first
    if let Some(start) = text.find("```json") {
        let start = start + 7;
        if let Some(end) = text[start..].find("```") {
            return Ok(text[start..start + end].trim().to_string());
        }
    }
    if let Some(start) = text.find("```") {
        let start = start + 3;
        // Skip any language identifier on the same line
        let start = text[start..].find('\n').map(|i| start + i + 1).unwrap_or(start);
        if let Some(end) = text[start..].find("```") {
            return Ok(text[start..start + end].trim().to_string());
        }
    }

    // Try to find raw JSON object
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return Ok(text[start..=end].to_string());
        }
    }

    anyhow::bail!("No JSON found in AI response")
}
