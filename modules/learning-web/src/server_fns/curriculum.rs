use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurriculumResult {
    pub curriculum_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumListItem {
    pub id: String,
    pub title: String,
    pub topic: String,
}

/// Create a new curriculum by calling the Restate ingress to invoke the CurriculumWorkflow
#[server]
pub async fn create_curriculum(topic: String) -> Result<CreateCurriculumResult, ServerFnError> {
    let ingress_url =
        std::env::var("RESTATE_INGRESS_URL").unwrap_or_else(|_| "http://localhost:8080".into());

    let workflow_id = uuid::Uuid::new_v4().to_string();
    let url = format!(
        "{}/CurriculumWorkflow/{}/run/send",
        ingress_url, workflow_id
    );

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .json(&serde_json::json!({ "topic": topic }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to call Restate: {}", e)))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Restate error: {}", body)));
    }

    // The /send endpoint returns immediately. Poll for completion.
    // For MVP, wait and poll the output endpoint
    let output_url = format!(
        "{}/restate/workflow/CurriculumWorkflow/{}/attach",
        ingress_url, workflow_id
    );

    let result: serde_json::Value = client
        .get(&output_url)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get workflow output: {}", e)))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse workflow output: {}", e)))?;

    Ok(CreateCurriculumResult {
        curriculum_id: result["curriculum_id"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        title: result["title"].as_str().unwrap_or("").to_string(),
    })
}

/// List all curricula from the API
#[server]
pub async fn list_curricula() -> Result<Vec<CurriculumListItem>, ServerFnError> {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let url = format!("{}/api/curricula", api_url);

    let client = reqwest::Client::new();
    let resp: Vec<serde_json::Value> = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list curricula: {}", e)))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse curricula: {}", e)))?;

    Ok(resp
        .iter()
        .map(|c| CurriculumListItem {
            id: c["id"].as_str().unwrap_or("").to_string(),
            title: c["title"].as_str().unwrap_or("").to_string(),
            topic: c["topic"].as_str().unwrap_or("").to_string(),
        })
        .collect())
}

/// Get a curriculum with its chapters
#[server]
pub async fn get_curriculum(id: String) -> Result<serde_json::Value, ServerFnError> {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let url = format!("{}/api/curricula/{}", api_url, id);

    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get curriculum: {}", e)))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse curriculum: {}", e)))?;

    Ok(resp)
}

/// Get a node with its edges, children, and videos
#[server]
pub async fn get_node(id: String) -> Result<serde_json::Value, ServerFnError> {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let url = format!("{}/api/nodes/{}", api_url, id);

    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get node: {}", e)))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse node: {}", e)))?;

    Ok(resp)
}
